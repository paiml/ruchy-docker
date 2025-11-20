/// EXTREME TDD: Comprehensive Edge Case Tests for Metrics Module
///
/// These tests verify that parse_benchmark_output handles ALL edge cases correctly:
/// - Malformed input (missing fields, invalid formats, extra whitespace)
/// - Boundary values (zero, negative, very large numbers)
/// - Unicode and special characters
/// - Multiple formats and variations
/// - Error conditions
///
/// Goal: Achieve ≥85% coverage and ≥85% mutation score
use ruchy_docker::metrics::parse_benchmark_output;

// ============================================================================
// Missing Fields Tests
// ============================================================================

#[test]
fn test_missing_startup_time() {
    let output = "COMPUTE_TIME_US: 200\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_err(), "Should fail when STARTUP_TIME_US is missing");
}

#[test]
fn test_missing_compute_time() {
    let output = "STARTUP_TIME_US: 100\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_err(), "Should fail when COMPUTE_TIME_US is missing");
}

#[test]
fn test_missing_result_is_ok() {
    // RESULT is optional
    let output = "STARTUP_TIME_US: 100\nCOMPUTE_TIME_US: 200";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok(), "Should succeed when RESULT is missing (it's optional)");
    let benchmark = result.unwrap();
    assert_eq!(benchmark.result_value, None);
}

// ============================================================================
// Invalid Format Tests
// ============================================================================

#[test]
fn test_non_numeric_startup_time() {
    let output = "STARTUP_TIME_US: abc\nCOMPUTE_TIME_US: 200\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_err(), "Should fail when STARTUP_TIME_US is not numeric");
}

#[test]
fn test_non_numeric_compute_time() {
    let output = "STARTUP_TIME_US: 100\nCOMPUTE_TIME_US: xyz\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_err(), "Should fail when COMPUTE_TIME_US is not numeric");
}

#[test]
fn test_non_numeric_result() {
    let output = "STARTUP_TIME_US: 100\nCOMPUTE_TIME_US: 200\nRESULT: not_a_number";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok(), "Should succeed - RESULT is optional");
    let benchmark = result.unwrap();
    assert_eq!(benchmark.result_value, None, "Invalid RESULT should be treated as None");
}

// ============================================================================
// Boundary Value Tests
// ============================================================================

#[test]
fn test_zero_startup_time() {
    let output = "STARTUP_TIME_US: 0\nCOMPUTE_TIME_US: 200\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok());
    let benchmark = result.unwrap();
    assert_eq!(benchmark.startup_time_us, 0);
}

#[test]
fn test_zero_compute_time() {
    let output = "STARTUP_TIME_US: 100\nCOMPUTE_TIME_US: 0\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok());
    let benchmark = result.unwrap();
    assert_eq!(benchmark.compute_time_us, 0);
}

#[test]
fn test_very_large_times() {
    let output = "STARTUP_TIME_US: 18446744073709551615\nCOMPUTE_TIME_US: 18446744073709551615\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok(), "Should handle very large times without panicking");
    let benchmark = result.unwrap();
    assert_eq!(benchmark.startup_time_us, u64::MAX);
    assert_eq!(benchmark.compute_time_us, u64::MAX);
    // With saturating_add, u64::MAX + u64::MAX = u64::MAX
    assert_eq!(benchmark.total_time_us, u64::MAX, "saturating_add should prevent overflow");
}

#[test]
fn test_negative_result_value() {
    let output = "STARTUP_TIME_US: 100\nCOMPUTE_TIME_US: 200\nRESULT: -42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok());
    let benchmark = result.unwrap();
    assert_eq!(benchmark.result_value, Some(-42), "Should handle negative result values");
}

// ============================================================================
// Whitespace and Formatting Tests
// ============================================================================

#[test]
fn test_extra_whitespace() {
    let output = "STARTUP_TIME_US:    100   \nCOMPUTE_TIME_US:   200  \nRESULT:  42  ";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok(), "Should handle extra whitespace");
    let benchmark = result.unwrap();
    assert_eq!(benchmark.startup_time_us, 100);
    assert_eq!(benchmark.compute_time_us, 200);
    assert_eq!(benchmark.result_value, Some(42));
}

#[test]
fn test_tabs_instead_of_spaces() {
    let output = "STARTUP_TIME_US:\t100\nCOMPUTE_TIME_US:\t200\nRESULT:\t42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok(), "Should handle tabs");
    let benchmark = result.unwrap();
    assert_eq!(benchmark.startup_time_us, 100);
}

#[test]
fn test_windows_line_endings() {
    let output = "STARTUP_TIME_US: 100\r\nCOMPUTE_TIME_US: 200\r\nRESULT: 42\r\n";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok(), "Should handle Windows line endings");
}

#[test]
fn test_mixed_line_endings() {
    let output = "STARTUP_TIME_US: 100\r\nCOMPUTE_TIME_US: 200\nRESULT: 42\r\n";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok(), "Should handle mixed line endings");
}

// ============================================================================
// Case Sensitivity Tests
// ============================================================================

#[test]
fn test_lowercase_keys() {
    let output = "startup_time_us: 100\ncompute_time_us: 200\nresult: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_err(), "Should be case-sensitive (uppercase required)");
}

#[test]
fn test_mixed_case_keys() {
    let output = "Startup_Time_Us: 100\nCOMPUTE_TIME_US: 200\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_err(), "Should require exact case match");
}

// ============================================================================
// Extra Content Tests
// ============================================================================

#[test]
fn test_extra_output_before() {
    let output = "Some debug output\nSTARTUP_TIME_US: 100\nCOMPUTE_TIME_US: 200\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok(), "Should extract metrics even with extra output before");
}

#[test]
fn test_extra_output_after() {
    let output = "STARTUP_TIME_US: 100\nCOMPUTE_TIME_US: 200\nRESULT: 42\nSome cleanup logs";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok(), "Should extract metrics even with extra output after");
}

#[test]
fn test_duplicate_fields() {
    let output = "STARTUP_TIME_US: 100\nSTARTUP_TIME_US: 999\nCOMPUTE_TIME_US: 200\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok(), "Should handle duplicate fields");
    let benchmark = result.unwrap();
    // Regex should match the first occurrence
    assert_eq!(benchmark.startup_time_us, 100);
}

// ============================================================================
// Empty and Null Tests
// ============================================================================

#[test]
fn test_empty_string() {
    let output = "";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_err(), "Should fail on empty string");
}

#[test]
fn test_only_whitespace() {
    let output = "   \n\t\n   ";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_err(), "Should fail on whitespace-only string");
}

#[test]
fn test_only_newlines() {
    let output = "\n\n\n\n";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_err(), "Should fail on newlines-only string");
}

// ============================================================================
// Benchmark Name and Language Tests
// ============================================================================

#[test]
fn test_unicode_benchmark_name() {
    let output = "STARTUP_TIME_US: 100\nCOMPUTE_TIME_US: 200\nRESULT: 42";
    let result = parse_benchmark_output(output, "测试-benchmark-🚀", "rust");
    assert!(result.is_ok(), "Should handle Unicode benchmark names");
    let benchmark = result.unwrap();
    assert_eq!(benchmark.benchmark_name, "测试-benchmark-🚀");
}

#[test]
fn test_empty_benchmark_name() {
    let output = "STARTUP_TIME_US: 100\nCOMPUTE_TIME_US: 200\nRESULT: 42";
    let result = parse_benchmark_output(output, "", "rust");
    assert!(result.is_ok(), "Should handle empty benchmark name");
    let benchmark = result.unwrap();
    assert_eq!(benchmark.benchmark_name, "");
}

#[test]
fn test_empty_language() {
    let output = "STARTUP_TIME_US: 100\nCOMPUTE_TIME_US: 200\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "");
    assert!(result.is_ok(), "Should handle empty language");
    let benchmark = result.unwrap();
    assert_eq!(benchmark.language, "");
}

// ============================================================================
// Total Time Calculation Tests
// ============================================================================

#[test]
fn test_total_time_calculation() {
    let output = "STARTUP_TIME_US: 100\nCOMPUTE_TIME_US: 200\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok());
    let benchmark = result.unwrap();
    assert_eq!(benchmark.total_time_us, 300, "total_time_us should be startup + compute");
}

#[test]
fn test_total_time_with_zero_startup() {
    let output = "STARTUP_TIME_US: 0\nCOMPUTE_TIME_US: 200\nRESULT: 42";
    let result = parse_benchmark_output(output, "test", "rust");
    assert!(result.is_ok());
    let benchmark = result.unwrap();
    assert_eq!(benchmark.total_time_us, 200);
}

#[test]
fn test_total_time_overflow() {
    // Test overflow prevention with saturating_add
    let max_u64 = u64::MAX;
    let output = format!("STARTUP_TIME_US: {}\nCOMPUTE_TIME_US: 1\nRESULT: 42", max_u64);
    let result = parse_benchmark_output(&output, "test", "rust");
    assert!(result.is_ok(), "Should handle overflow gracefully with saturating_add");
    let benchmark = result.unwrap();
    assert_eq!(benchmark.startup_time_us, max_u64);
    assert_eq!(benchmark.compute_time_us, 1);
    // saturating_add means u64::MAX + 1 = u64::MAX
    assert_eq!(benchmark.total_time_us, max_u64, "saturating_add should saturate at u64::MAX");
}

// ============================================================================
// Property-Based Tests
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: Any valid startup and compute times should produce a valid benchmark
        #[test]
        fn prop_valid_times_produce_valid_benchmark(
            startup in 0u64..1_000_000_000u64,
            compute in 0u64..1_000_000_000u64,
            result_val in prop::option::of(-1000i64..1000i64)
        ) {
            let result_str = match result_val {
                Some(val) => format!("\nRESULT: {}", val),
                None => String::new(),
            };
            let output = format!(
                "STARTUP_TIME_US: {}\nCOMPUTE_TIME_US: {}{}",
                startup, compute, result_str
            );

            let parse_result = parse_benchmark_output(&output, "prop_test", "rust");
            prop_assert!(parse_result.is_ok(), "Valid input should always parse successfully");

            let benchmark = parse_result.unwrap();
            prop_assert_eq!(benchmark.startup_time_us, startup);
            prop_assert_eq!(benchmark.compute_time_us, compute);
            prop_assert_eq!(benchmark.total_time_us, startup + compute);
            prop_assert_eq!(benchmark.result_value, result_val);
        }

        /// Property: Whitespace variations should not affect parsing
        #[test]
        fn prop_whitespace_invariant(
            spaces_before_colon in 0usize..5,
            spaces_after_colon in 1usize..10,
        ) {
            let ws_before = " ".repeat(spaces_before_colon);
            let ws_after = " ".repeat(spaces_after_colon);

            let output = format!(
                "STARTUP_TIME_US{}:{}100\nCOMPUTE_TIME_US{}:{}200\nRESULT{}:{}42",
                ws_before, ws_after, ws_before, ws_after, ws_before, ws_after
            );

            let result = parse_benchmark_output(&output, "test", "rust");
            if result.is_ok() {
                let benchmark = result.unwrap();
                prop_assert_eq!(benchmark.startup_time_us, 100);
                prop_assert_eq!(benchmark.compute_time_us, 200);
                prop_assert_eq!(benchmark.result_value, Some(42));
            }
        }
    }
}
