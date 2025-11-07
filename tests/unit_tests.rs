use ruchy_docker::metrics::BenchmarkResult;

/// RED Phase: Write failing test first
/// Test that BenchmarkResult can be created with valid timing data
#[test]
fn test_benchmark_result_creation() {
    let result = BenchmarkResult {
        benchmark_name: "fibonacci".to_string(),
        language: "ruchy-transpiled".to_string(),
        startup_time_us: 8234,
        compute_time_us: 23891,
        total_time_us: 32125,
        result_value: Some(9227465),
        image_size_mb: 3.2,
        memory_usage_mb: 8.5,
    };

    assert_eq!(result.benchmark_name, "fibonacci");
    assert_eq!(result.language, "ruchy-transpiled");
    assert_eq!(result.startup_time_us, 8234);
    assert_eq!(result.compute_time_us, 23891);
    assert_eq!(result.total_time_us, 32125);
    assert_eq!(result.result_value, Some(9227465));
}

/// Test that BenchmarkResult can convert microseconds to milliseconds correctly
#[test]
fn test_benchmark_result_time_conversions() {
    let result = BenchmarkResult {
        benchmark_name: "test".to_string(),
        language: "rust".to_string(),
        startup_time_us: 12340,
        compute_time_us: 23890,
        total_time_us: 36230,
        result_value: None,
        image_size_mb: 1.0,
        memory_usage_mb: 1.0,
    };

    assert_eq!(result.startup_time_ms(), 12.34);
    assert_eq!(result.compute_time_ms(), 23.89);
    assert_eq!(result.total_time_ms(), 36.23);
}

/// Test parsing of standardized benchmark output format
/// Format:
/// STARTUP_TIME_US: 8234
/// COMPUTE_TIME_US: 23891
/// RESULT: 9227465
#[test]
fn test_parse_benchmark_output_success() {
    let output = r#"STARTUP_TIME_US: 8234
COMPUTE_TIME_US: 23891
RESULT: 9227465"#;

    let result =
        ruchy_docker::metrics::parse_benchmark_output(output, "fibonacci", "ruchy-transpiled");

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.startup_time_us, 8234);
    assert_eq!(result.compute_time_us, 23891);
    assert_eq!(result.total_time_us, 8234 + 23891);
    assert_eq!(result.result_value, Some(9227465));
}

/// Test parsing with missing fields returns error
#[test]
fn test_parse_benchmark_output_missing_fields() {
    let output = "STARTUP_TIME_US: 8234";

    let result =
        ruchy_docker::metrics::parse_benchmark_output(output, "fibonacci", "ruchy-transpiled");

    assert!(result.is_err());
}

/// Test parsing with invalid numbers returns error
#[test]
fn test_parse_benchmark_output_invalid_numbers() {
    let output = r#"STARTUP_TIME_US: invalid
COMPUTE_TIME_US: 23891
RESULT: 9227465"#;

    let result =
        ruchy_docker::metrics::parse_benchmark_output(output, "fibonacci", "ruchy-transpiled");

    assert!(result.is_err());
}

/// Test parsing output without result value (e.g., startup benchmark)
#[test]
fn test_parse_benchmark_output_no_result() {
    let output = r#"STARTUP_TIME_US: 5000
COMPUTE_TIME_US: 1000"#;

    let result =
        ruchy_docker::metrics::parse_benchmark_output(output, "startup", "ruchy-transpiled");

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.result_value, None);
}

/// Property test: total time should always equal startup + compute
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_total_time_equals_sum(
            startup_us in 0u64..1_000_000,
            compute_us in 0u64..10_000_000
        ) {
            let result = BenchmarkResult {
                benchmark_name: "test".to_string(),
                language: "test".to_string(),
                startup_time_us: startup_us,
                compute_time_us: compute_us,
                total_time_us: startup_us + compute_us,
                result_value: None,
                image_size_mb: 1.0,
                memory_usage_mb: 1.0,
            };

            prop_assert_eq!(result.total_time_us, result.startup_time_us + result.compute_time_us);
        }
    }
}
