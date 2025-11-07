/// RED Phase: Tests for statistical analyzer
///
/// These tests validate the statistical analysis methods used for
/// aggregating and comparing benchmark results across languages.
///
/// Key metrics:
/// - Geometric Mean: (∏ xi)^(1/n) - Traditional benchmark metric
/// - Arithmetic Mean: (∑ xi) / n - Total CPU time
/// - Harmonic Mean: n / (∑ 1/xi) - Average speedup
/// - MAD Outlier Detection: Median Absolute Deviation method
use ruchy_docker::analyzer::{
    calculate_arithmetic_mean, calculate_geometric_mean, calculate_harmonic_mean,
    detect_outliers_mad, AggregationMetrics,
};

/// Test geometric mean calculation
/// Formula: (∏ xi)^(1/n)
/// Example: geomean([2, 8]) = sqrt(2*8) = 4
#[test]
fn test_geometric_mean_simple() {
    let values = vec![2.0, 8.0];
    let result = calculate_geometric_mean(&values);

    assert!(result.is_ok());
    let mean = result.unwrap();
    assert!((mean - 4.0).abs() < 0.0001, "geomean([2, 8]) should be 4.0");
}

/// Test geometric mean with identical values
#[test]
fn test_geometric_mean_identical() {
    let values = vec![5.0, 5.0, 5.0, 5.0];
    let result = calculate_geometric_mean(&values);

    assert!(result.is_ok());
    let mean = result.unwrap();
    assert!(
        (mean - 5.0).abs() < 0.0001,
        "geomean of identical values should equal that value"
    );
}

/// Test geometric mean with benchmark speedups
/// Example: Language A is 2x faster, Language B is 4x faster
/// Geometric mean speedup: sqrt(2 * 4) = 2.83x
#[test]
fn test_geometric_mean_speedups() {
    let speedups = vec![2.0, 4.0];
    let result = calculate_geometric_mean(&speedups);

    assert!(result.is_ok());
    let mean = result.unwrap();
    assert!(
        (mean - 2.8284).abs() < 0.01,
        "geomean speedup should be ~2.83x"
    );
}

/// Test geometric mean with empty input returns error
#[test]
fn test_geometric_mean_empty() {
    let values: Vec<f64> = vec![];
    let result = calculate_geometric_mean(&values);

    assert!(result.is_err(), "Empty input should return error");
}

/// Test geometric mean with zero value returns error
#[test]
fn test_geometric_mean_with_zero() {
    let values = vec![1.0, 0.0, 3.0];
    let result = calculate_geometric_mean(&values);

    assert!(
        result.is_err(),
        "Geometric mean with zero should return error"
    );
}

/// Test arithmetic mean calculation
/// Formula: (∑ xi) / n
#[test]
fn test_arithmetic_mean_simple() {
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = calculate_arithmetic_mean(&values);

    assert!(result.is_ok());
    let mean = result.unwrap();
    assert!((mean - 3.0).abs() < 0.0001, "arithmetic mean should be 3.0");
}

/// Test arithmetic mean represents total CPU time
#[test]
fn test_arithmetic_mean_cpu_time() {
    // Three benchmarks: 10ms, 20ms, 30ms
    let times = vec![10.0, 20.0, 30.0];
    let result = calculate_arithmetic_mean(&times);

    assert!(result.is_ok());
    let mean = result.unwrap();
    assert!(
        (mean - 20.0).abs() < 0.0001,
        "arithmetic mean should be 20ms"
    );
}

/// Test harmonic mean calculation
/// Formula: n / (∑ 1/xi)
#[test]
fn test_harmonic_mean_simple() {
    let values = vec![1.0, 2.0, 4.0];
    let result = calculate_harmonic_mean(&values);

    assert!(result.is_ok());
    let mean = result.unwrap();
    // harmonic mean = 3 / (1/1 + 1/2 + 1/4) = 3 / 1.75 = 1.714
    assert!((mean - 1.714).abs() < 0.01, "harmonic mean should be ~1.71");
}

/// Test harmonic mean for speedup calculations
/// If baseline takes 100ms and optimized takes 50ms, speedup is 2x
#[test]
fn test_harmonic_mean_speedups() {
    let speedups = vec![2.0, 4.0, 8.0];
    let result = calculate_harmonic_mean(&speedups);

    assert!(result.is_ok());
    let mean = result.unwrap();
    // harmonic mean = 3 / (1/2 + 1/4 + 1/8) = 3 / 0.875 = 3.43
    assert!(
        (mean - 3.43).abs() < 0.01,
        "harmonic mean speedup should be ~3.43x"
    );
}

/// Test MAD-based outlier detection
/// MAD = Median Absolute Deviation from median
/// Outlier threshold: median ± k * 1.4826 * MAD (k=3 typical)
#[test]
fn test_mad_outlier_detection_no_outliers() {
    let values = vec![10.0, 11.0, 10.5, 10.2, 10.8];
    let outliers = detect_outliers_mad(&values, 3.0);

    assert!(outliers.is_empty(), "No outliers in tight distribution");
}

/// Test MAD outlier detection with clear outlier
#[test]
fn test_mad_outlier_detection_with_outlier() {
    let values = vec![10.0, 11.0, 10.5, 10.2, 100.0]; // 100.0 is outlier
    let outliers = detect_outliers_mad(&values, 3.0);

    assert!(!outliers.is_empty(), "Should detect outlier");
    assert!(
        outliers.contains(&4),
        "Index 4 (100.0) should be flagged as outlier"
    );
}

/// Test MAD outlier detection with multiple outliers
#[test]
fn test_mad_outlier_detection_multiple() {
    let values = vec![10.0, 11.0, 100.0, 10.5, 200.0, 10.2];
    let outliers = detect_outliers_mad(&values, 3.0);

    assert!(outliers.len() >= 2, "Should detect multiple outliers");
    assert!(outliers.contains(&2), "Index 2 (100.0) should be outlier");
    assert!(outliers.contains(&4), "Index 4 (200.0) should be outlier");
}

/// Test complete aggregation metrics calculation
#[test]
fn test_aggregation_metrics_complete() {
    let values = vec![10.0, 20.0, 30.0, 40.0];

    let metrics = AggregationMetrics::from_values(&values);

    assert!(metrics.is_ok());
    let m = metrics.unwrap();

    // Geometric mean: (10*20*30*40)^(1/4) = 22.134
    assert!((m.geometric_mean - 22.134).abs() < 0.01);

    // Arithmetic mean: (10+20+30+40)/4 = 25.0
    assert!((m.arithmetic_mean - 25.0).abs() < 0.01);

    // Harmonic mean: 4 / (1/10 + 1/20 + 1/30 + 1/40) = 19.2
    assert!((m.harmonic_mean - 19.2).abs() < 0.1);

    // No outliers in uniform distribution
    assert!(m.outlier_indices.is_empty());
}

/// Property test: Geometric mean is always between min and max
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_geometric_mean_bounded(values in prop::collection::vec(1.0f64..1000.0, 2..20)) {
            let result = calculate_geometric_mean(&values);
            if let Ok(mean) = result {
                let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
                let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                prop_assert!(mean >= min && mean <= max,
                    "Geometric mean must be between min and max");
            }
        }

        #[test]
        fn prop_arithmetic_mean_equals_sum_divided_by_count(
            values in prop::collection::vec(1.0f64..1000.0, 1..20)
        ) {
            let result = calculate_arithmetic_mean(&values);
            let sum: f64 = values.iter().sum();
            let expected = sum / values.len() as f64;

            if let Ok(mean) = result {
                prop_assert!((mean - expected).abs() < 0.0001,
                    "Arithmetic mean must equal sum/n");
            }
        }
    }
}
