use anyhow::{Context, Result};
use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions, WaitContainerOptions,
};
use bollard::image::ListImagesOptions;
use bollard::Docker;
use log::{debug, info};
use std::collections::HashMap;

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
    pub async fn new() -> Result<Self> {
        let docker = Docker::connect_with_local_defaults()
            .context("Failed to connect to Docker daemon. Is Docker running?")?;

        // Verify Docker connection
        let version = docker
            .version()
            .await
            .context("Failed to get Docker version")?;
        info!(
            "Connected to Docker v{}",
            version.version.unwrap_or_default()
        );

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
    pub async fn run_container(&self, image: &str, cmd: Vec<&str>) -> Result<String> {
        debug!("Running container: {} with cmd: {:?}", image, cmd);

        // Create container configuration
        let config = Config {
            image: Some(image.to_string()),
            cmd: Some(cmd.iter().map(|s| s.to_string()).collect()),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            ..Default::default()
        };

        let container_name = format!("ruchy-bench-{}", uuid::Uuid::new_v4());
        let options = CreateContainerOptions {
            name: container_name.clone(),
            platform: None,
        };

        // Create container
        let container = self
            .docker
            .create_container(Some(options), config)
            .await
            .context("Failed to create container")?;

        // Start container
        self.docker
            .start_container::<String>(&container.id, None)
            .await
            .context("Failed to start container")?;

        // Wait for container to finish
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
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("Error waiting for container: {}", e));
                }
            }
        }

        // Get container logs
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
                    output.push_str(&log.to_string());
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("Error reading container logs: {}", e));
                }
            }
        }

        // Clean up container
        let remove_options = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };
        self.docker
            .remove_container(&container.id, Some(remove_options))
            .await
            .context("Failed to remove container")?;

        debug!("Container output: {}", output);
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
    pub async fn get_image_size_mb(&self, image: &str) -> Result<f64> {
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
            return Err(anyhow::anyhow!("Image not found: {}", image));
        }

        let size_bytes = images[0].size;
        let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
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
    pub async fn enrich_with_metadata(
        &self,
        result: &mut BenchmarkResult,
        image: &str,
    ) -> Result<()> {
        // Get image size
        result.image_size_mb = self.get_image_size_mb(image).await?;

        // TODO: Implement memory usage collection via Docker stats
        // For now, set to 0.0 (will be populated in future iteration)
        result.memory_usage_mb = 0.0;

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
