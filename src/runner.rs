use anyhow::{Context, Result};
use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions, WaitContainerOptions,
};
use bollard::image::ListImagesOptions;
use bollard::Docker;
use log::{debug, info};
use std::collections::HashMap;
use tracing::{debug as trace_debug, info as trace_info, instrument, trace, warn};

use crate::BenchmarkResult;

/// Docker container orchestration and benchmark execution
///
/// This module handles:
/// - Docker image building and management
/// - Container execution with resource limits
/// - Output capture and parsing
/// - Environment variable injection
/// - Volume mounting for I/O benchmarks
pub struct BenchmarkRunner {
    docker: Docker,
}

impl BenchmarkRunner {
    /// Create a new BenchmarkRunner connected to Docker daemon
    #[instrument]
    pub async fn new() -> Result<Self> {
        trace!("connecting to Docker daemon");
        let docker = Docker::connect_with_local_defaults()
            .context("Failed to connect to Docker daemon. Is Docker running?")?;

        trace!("fetching Docker version");
        // Verify Docker connection
        let version = docker
            .version()
            .await
            .context("Failed to get Docker version")?;
        let version_str = version.version.unwrap_or_default();
        info!("Connected to Docker v{}", version_str);
        trace_info!(version = %version_str, "Docker connection established");

        Ok(Self { docker })
    }

    /// Run a Docker container and capture stdout
    ///
    /// # Arguments
    /// * `image` - Docker image name (e.g., "alpine:latest")
    /// * `cmd` - Command to execute in container
    ///
    /// # Returns
    /// * `Ok(String)` - Container stdout output
    /// * `Err(_)` - Error if container fails or times out
    #[instrument(skip(self), fields(image = %image, cmd = ?cmd))]
    pub async fn run_container(&self, image: &str, cmd: Vec<&str>) -> Result<String> {
        debug!("Running container: {} with cmd: {:?}", image, cmd);
        trace!("preparing container configuration");

        // Create container configuration
        let config = Config {
            image: Some(image.to_string()),
            cmd: Some(cmd.iter().map(|s| s.to_string()).collect()),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            ..Default::default()
        };

        let container_name = format!("ruchy-bench-{}", uuid::Uuid::new_v4());
        trace_debug!(container_name = %container_name, "generated container name");
        let options = CreateContainerOptions {
            name: container_name.clone(),
            platform: None,
        };

        // Create container
        trace!("creating container");
        let container = self
            .docker
            .create_container(Some(options), config)
            .await
            .context("Failed to create container")?;
        trace_debug!(container_id = %container.id, "container created");

        // Start container
        trace!("starting container");
        self.docker
            .start_container::<String>(&container.id, None)
            .await
            .context("Failed to start container")?;
        trace_debug!(container_id = %container.id, "container started");

        // Wait for container to finish
        trace!("waiting for container to finish");
        let wait_options = WaitContainerOptions {
            condition: "not-running",
        };
        let mut wait_stream = self
            .docker
            .wait_container(&container.id, Some(wait_options));

        // Collect wait results
        use futures_util::StreamExt;
        while let Some(result) = wait_stream.next().await {
            match result {
                Ok(wait_response) => {
                    debug!("Container wait response: {:?}", wait_response);
                    trace_debug!(response = ?wait_response, "received wait response");
                }
                Err(e) => {
                    warn!(error = %e, "error waiting for container");
                    return Err(anyhow::anyhow!("Error waiting for container: {}", e));
                }
            }
        }
        trace_debug!("container finished execution");

        // Get container logs
        trace!("fetching container logs");
        use bollard::container::LogsOptions;
        let log_options = LogsOptions::<String> {
            stdout: true,
            stderr: true,
            follow: false,
            ..Default::default()
        };

        let mut log_stream = self.docker.logs(&container.id, Some(log_options));
        let mut output = String::new();

        while let Some(log_result) = log_stream.next().await {
            match log_result {
                Ok(log) => {
                    let log_chunk = log.to_string();
                    trace!(bytes = log_chunk.len(), "received log chunk");
                    output.push_str(&log_chunk);
                }
                Err(e) => {
                    warn!(error = %e, "error reading container logs");
                    return Err(anyhow::anyhow!("Error reading container logs: {}", e));
                }
            }
        }
        trace_debug!(output_size = output.len(), "collected all container logs");

        // Clean up container
        trace!("removing container");
        let remove_options = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };
        self.docker
            .remove_container(&container.id, Some(remove_options))
            .await
            .context("Failed to remove container")?;
        trace_debug!("container removed");

        debug!("Container output: {}", output);
        trace_info!(output_lines = output.lines().count(), "container execution complete");
        Ok(output)
    }

    /// Get Docker image size in megabytes
    ///
    /// # Arguments
    /// * `image` - Docker image name
    ///
    /// # Returns
    /// * `Ok(f64)` - Image size in MB
    /// * `Err(_)` - Error if image not found
    #[instrument(skip(self), fields(image = %image))]
    pub async fn get_image_size_mb(&self, image: &str) -> Result<f64> {
        trace!("listing Docker images");
        let mut filters = HashMap::new();
        filters.insert("reference".to_string(), vec![image.to_string()]);

        let options = ListImagesOptions {
            all: false,
            filters,
            ..Default::default()
        };

        let images = self
            .docker
            .list_images(Some(options))
            .await
            .context("Failed to list Docker images")?;

        if images.is_empty() {
            warn!(image = %image, "image not found");
            return Err(anyhow::anyhow!("Image not found: {}", image));
        }

        let size_bytes = images[0].size;
        let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
        trace_debug!(size_mb = %size_mb, "retrieved image size");
        Ok(size_mb)
    }

    /// Enrich a BenchmarkResult with Docker metadata (image size, memory usage)
    ///
    /// # Arguments
    /// * `result` - Mutable reference to BenchmarkResult to enrich
    /// * `image` - Docker image name
    ///
    /// # Returns
    /// * `Ok(())` - Success
    /// * `Err(_)` - Error if metadata collection fails
    #[instrument(skip(self, result), fields(image = %image))]
    pub async fn enrich_with_metadata(
        &self,
        result: &mut BenchmarkResult,
        image: &str,
    ) -> Result<()> {
        trace!("enriching benchmark result with metadata");
        // Get image size
        result.image_size_mb = self.get_image_size_mb(image).await?;
        trace_debug!(image_size_mb = %result.image_size_mb, "image size retrieved");

        // TODO: Implement memory usage collection via Docker stats
        // For now, set to 0.0 (will be populated in future iteration)
        result.memory_usage_mb = 0.0;
        trace!("metadata enrichment complete");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_runner_creation() {
        // This test requires Docker to be running
        // Skip if Docker is not available
        if Docker::connect_with_local_defaults().is_err() {
            eprintln!("Skipping test: Docker not available");
            return;
        }

        let runner = BenchmarkRunner::new().await;
        assert!(
            runner.is_ok(),
            "Should create runner when Docker is available"
        );
    }
}
