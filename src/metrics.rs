use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument, trace, warn};

/// Represents the result of a single benchmark execution
///
/// This struct captures all metrics from a Docker container benchmark run,
/// including instrumented timing data (startup vs compute), result validation,
/// and resource usage (image size, memory).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Benchmark name (e.g., "fibonacci", "primes", "array-sum")
    pub benchmark_name: String,

    /// Language implementation (e.g., "ruchy-transpiled", "rust", "python")
    pub language: String,

    /// Application startup time in microseconds
    /// Measured from process start to ready state (imports, allocations)
    pub startup_time_us: u64,

    /// Pure computation time in microseconds
    /// Measured for the actual benchmark algorithm execution
    pub compute_time_us: u64,

    /// Total time in microseconds (startup + compute)
    pub total_time_us: u64,

    /// Optional result value for validation (e.g., fib(35) = 9227465)
    pub result_value: Option<i64>,

    /// Docker image size in megabytes
    pub image_size_mb: f64,

    /// Peak memory usage in megabytes
    pub memory_usage_mb: f64,
}

impl BenchmarkResult {
    /// Convert startup time from microseconds to milliseconds
    pub fn startup_time_ms(&self) -> f64 {
        self.startup_time_us as f64 / 1000.0
    }

    /// Convert compute time from microseconds to milliseconds
    pub fn compute_time_ms(&self) -> f64 {
        self.compute_time_us as f64 / 1000.0
    }

    /// Convert total time from microseconds to milliseconds
    pub fn total_time_ms(&self) -> f64 {
        self.total_time_us as f64 / 1000.0
    }
}

/// Parse standardized benchmark output format
///
/// Expected format:
/// ```text
/// STARTUP_TIME_US: 8234
/// COMPUTE_TIME_US: 23891
/// RESULT: 9227465
/// ```
///
/// The RESULT field is optional (used for validation in compute benchmarks,
/// but not needed for startup-only benchmarks).
///
/// # Arguments
/// * `output` - Raw stdout from Docker container execution
/// * `benchmark_name` - Name of the benchmark being parsed
/// * `language` - Language implementation identifier
///
/// # Returns
/// * `Ok(BenchmarkResult)` - Parsed benchmark result with all metrics
/// * `Err(_)` - Parse error if format is invalid or required fields missing
#[instrument(skip(output), fields(benchmark_name = %benchmark_name, language = %language, output_len = output.len()))]
pub fn parse_benchmark_output(
    output: &str,
    benchmark_name: &str,
    language: &str,
) -> Result<BenchmarkResult> {
    trace!("parsing benchmark output");
    // Compile regex patterns for extracting metrics
    trace!("compiling regex patterns");
    let startup_re =
        Regex::new(r"STARTUP_TIME_US:\s*(\d+)").context("Failed to compile startup time regex")?;
    let compute_re =
        Regex::new(r"COMPUTE_TIME_US:\s*(\d+)").context("Failed to compile compute time regex")?;
    let result_re = Regex::new(r"RESULT:\s*(-?\d+)").context("Failed to compile result regex")?;

    // Extract startup time (required)
    trace!("extracting startup time");
    let startup_time_us = startup_re
        .captures(output)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<u64>().ok())
        .context("Missing or invalid STARTUP_TIME_US field")?;
    debug!(startup_time_us = %startup_time_us, "parsed startup time");

    // Extract compute time (required)
    trace!("extracting compute time");
    let compute_time_us = compute_re
        .captures(output)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<u64>().ok())
        .context("Missing or invalid COMPUTE_TIME_US field")?;
    debug!(compute_time_us = %compute_time_us, "parsed compute time");

    // Extract result value (optional)
    trace!("extracting result value");
    let result_value = result_re
        .captures(output)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<i64>().ok());

    if let Some(val) = result_value {
        debug!(result_value = %val, "parsed result value");
    } else {
        trace!("no result value found (optional field)");
    }

    // Calculate total time (use saturating_add to prevent overflow)
    let total_time_us = startup_time_us.saturating_add(compute_time_us);
    debug!(total_time_us = %total_time_us, "calculated total time");

    trace!("benchmark output parsing complete");
    Ok(BenchmarkResult {
        benchmark_name: benchmark_name.to_string(),
        language: language.to_string(),
        startup_time_us,
        compute_time_us,
        total_time_us,
        result_value,
        image_size_mb: 0.0,   // Will be populated by Docker inspection
        memory_usage_mb: 0.0, // Will be populated by Docker stats
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_conversions() {
        let result = BenchmarkResult {
            benchmark_name: "test".to_string(),
            language: "rust".to_string(),
            startup_time_us: 1000,
            compute_time_us: 2500,
            total_time_us: 3500,
            result_value: None,
            image_size_mb: 1.0,
            memory_usage_mb: 1.0,
        };

        assert_eq!(result.startup_time_ms(), 1.0);
        assert_eq!(result.compute_time_ms(), 2.5);
        assert_eq!(result.total_time_ms(), 3.5);
    }

    #[test]
    fn test_parse_complete_output() {
        let output = "STARTUP_TIME_US: 100\nCOMPUTE_TIME_US: 200\nRESULT: 42";
        let result = parse_benchmark_output(output, "test", "rust").unwrap();

        assert_eq!(result.startup_time_us, 100);
        assert_eq!(result.compute_time_us, 200);
        assert_eq!(result.total_time_us, 300);
        assert_eq!(result.result_value, Some(42));
    }

    #[test]
    fn test_parse_without_result() {
        let output = "STARTUP_TIME_US: 100\nCOMPUTE_TIME_US: 200";
        let result = parse_benchmark_output(output, "startup", "rust").unwrap();

        assert_eq!(result.result_value, None);
    }

    #[test]
    fn test_parse_missing_required_field() {
        let output = "STARTUP_TIME_US: 100";
        let result = parse_benchmark_output(output, "test", "rust");

        assert!(result.is_err());
    }
}
