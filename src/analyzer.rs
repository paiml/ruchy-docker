use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument, trace};

/// Statistical analysis of benchmark results
///
/// This module implements multiple aggregation metrics for comparing
/// programming language performance across benchmarks:
///
/// 1. **Geometric Mean** (∏ xi)^(1/n)
///    - Traditional benchmark metric (SPEC CPU, DaCapo)
///    - Prevents single benchmark from dominating results
///    - Appropriate for ratios and speedups
///
/// 2. **Arithmetic Mean** (∑ xi) / n
///    - Represents total CPU time across benchmarks
///    - Useful for understanding aggregate cost
///
/// 3. **Harmonic Mean** n / (∑ 1/xi)
///    - Average speedup metric
///    - Appropriate when averaging rates (throughput)
///
/// 4. **MAD Outlier Detection**
///    - Median Absolute Deviation method
///    - More robust than standard deviation for non-normal distributions
///    - Threshold: median ± k * 1.4826 * MAD (typically k=3)
///
/// Aggregation metrics for benchmark results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregationMetrics {
    /// Geometric mean: (∏ xi)^(1/n)
    pub geometric_mean: f64,

    /// Arithmetic mean: (∑ xi) / n
    pub arithmetic_mean: f64,

    /// Harmonic mean: n / (∑ 1/xi)
    pub harmonic_mean: f64,

    /// Indices of detected outliers (using MAD method)
    pub outlier_indices: Vec<usize>,

    /// Median value
    pub median: f64,

    /// Median Absolute Deviation
    pub mad: f64,
}

impl AggregationMetrics {
    /// Calculate all aggregation metrics from values
    ///
    /// # Arguments
    /// * `values` - Slice of f64 values (e.g., benchmark times or speedups)
    ///
    /// # Returns
    /// * `Ok(AggregationMetrics)` - Computed metrics
    /// * `Err(_)` - Error if values are invalid (empty, negative, etc.)
    #[instrument(skip(values), fields(count = values.len()))]
    pub fn from_values(values: &[f64]) -> Result<Self> {
        trace!("calculating aggregation metrics");

        trace!("calculating geometric mean");
        let geometric_mean = calculate_geometric_mean(values)?;
        debug!(geometric_mean = %geometric_mean, "computed geometric mean");

        trace!("calculating arithmetic mean");
        let arithmetic_mean = calculate_arithmetic_mean(values)?;
        debug!(arithmetic_mean = %arithmetic_mean, "computed arithmetic mean");

        trace!("calculating harmonic mean");
        let harmonic_mean = calculate_harmonic_mean(values)?;
        debug!(harmonic_mean = %harmonic_mean, "computed harmonic mean");

        trace!("calculating median and MAD");
        let median = calculate_median(values)?;
        let mad = calculate_mad(values, median)?;
        debug!(median = %median, mad = %mad, "computed median and MAD");

        trace!("detecting outliers");
        let outlier_indices = detect_outliers_mad(values, 3.0);
        debug!(outlier_count = outlier_indices.len(), "detected outliers");

        trace!("aggregation metrics calculation complete");
        Ok(Self {
            geometric_mean,
            arithmetic_mean,
            harmonic_mean,
            outlier_indices,
            median,
            mad,
        })
    }
}

/// Calculate geometric mean: (∏ xi)^(1/n)
///
/// The geometric mean is the traditional metric for benchmark aggregation
/// (Fleming & Wallace 1986, SPEC benchmarks). It prevents any single
/// benchmark from dominating the aggregate result.
///
/// # Arguments
/// * `values` - Slice of positive f64 values
///
/// # Returns
/// * `Ok(f64)` - Geometric mean
/// * `Err(_)` - Error if values are empty, contain zero/negative values
#[instrument(skip(values), fields(count = values.len()))]
pub fn calculate_geometric_mean(values: &[f64]) -> Result<f64> {
    if values.is_empty() {
        return Err(anyhow::anyhow!(
            "Cannot calculate geometric mean of empty values"
        ));
    }

    // Check for non-positive values
    for &v in values {
        if v <= 0.0 {
            return Err(anyhow::anyhow!(
                "Geometric mean requires positive values, found: {}",
                v
            ));
        }
    }

    // Calculate product in log space to avoid overflow
    // geomean = exp((1/n) * sum(log(xi)))
    trace!("calculating in log space to avoid overflow");
    let sum_log: f64 = values.iter().map(|x| x.ln()).sum();
    let mean_log = sum_log / values.len() as f64;
    let geometric_mean = mean_log.exp();

    Ok(geometric_mean)
}

/// Calculate arithmetic mean: (∑ xi) / n
///
/// The arithmetic mean represents the average value across benchmarks.
/// For timing data, it represents the total CPU time cost.
///
/// # Arguments
/// * `values` - Slice of f64 values
///
/// # Returns
/// * `Ok(f64)` - Arithmetic mean
/// * `Err(_)` - Error if values are empty
pub fn calculate_arithmetic_mean(values: &[f64]) -> Result<f64> {
    if values.is_empty() {
        return Err(anyhow::anyhow!(
            "Cannot calculate arithmetic mean of empty values"
        ));
    }

    let sum: f64 = values.iter().sum();
    let mean = sum / values.len() as f64;

    Ok(mean)
}

/// Calculate harmonic mean: n / (∑ 1/xi)
///
/// The harmonic mean is appropriate for averaging rates (e.g., throughput)
/// or speedup factors. It gives more weight to smaller values.
///
/// # Arguments
/// * `values` - Slice of positive f64 values
///
/// # Returns
/// * `Ok(f64)` - Harmonic mean
/// * `Err(_)` - Error if values are empty or contain zero
pub fn calculate_harmonic_mean(values: &[f64]) -> Result<f64> {
    if values.is_empty() {
        return Err(anyhow::anyhow!(
            "Cannot calculate harmonic mean of empty values"
        ));
    }

    // Check for zero values
    for &v in values {
        if v == 0.0 {
            return Err(anyhow::anyhow!("Harmonic mean undefined for zero values"));
        }
    }

    let sum_reciprocals: f64 = values.iter().map(|x| 1.0 / x).sum();
    let harmonic_mean = values.len() as f64 / sum_reciprocals;

    Ok(harmonic_mean)
}

/// Calculate median value
///
/// # Arguments
/// * `values` - Slice of f64 values
///
/// # Returns
/// * `Ok(f64)` - Median value
/// * `Err(_)` - Error if values are empty
fn calculate_median(values: &[f64]) -> Result<f64> {
    if values.is_empty() {
        return Err(anyhow::anyhow!("Cannot calculate median of empty values"));
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = sorted.len() / 2;
    let median = if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    };

    Ok(median)
}

/// Calculate Median Absolute Deviation (MAD)
///
/// MAD is a robust measure of variability, more resistant to outliers
/// than standard deviation.
///
/// Formula: MAD = median(|xi - median(x)|)
///
/// # Arguments
/// * `values` - Slice of f64 values
/// * `median` - Pre-computed median value
///
/// # Returns
/// * `Ok(f64)` - MAD value
/// * `Err(_)` - Error if values are empty
fn calculate_mad(values: &[f64], median: f64) -> Result<f64> {
    if values.is_empty() {
        return Err(anyhow::anyhow!("Cannot calculate MAD of empty values"));
    }

    let deviations: Vec<f64> = values.iter().map(|x| (x - median).abs()).collect();
    calculate_median(&deviations)
}

/// Detect outliers using Median Absolute Deviation (MAD) method
///
/// This is more robust than z-score for non-normal distributions.
/// Threshold: |xi - median| > k * 1.4826 * MAD
///
/// The constant 1.4826 is used to make MAD consistent with standard
/// deviation for normal distributions.
///
/// # Arguments
/// * `values` - Slice of f64 values
/// * `k` - Number of MADs from median (typically 2.5 or 3.0)
///
/// # Returns
/// * `Vec<usize>` - Indices of detected outliers
pub fn detect_outliers_mad(values: &[f64], k: f64) -> Vec<usize> {
    if values.is_empty() {
        return vec![];
    }

    let median = match calculate_median(values) {
        Ok(m) => m,
        Err(_) => return vec![],
    };

    let mad = match calculate_mad(values, median) {
        Ok(m) => m,
        Err(_) => return vec![],
    };

    // If MAD is zero, all values are identical (no outliers)
    if mad == 0.0 {
        return vec![];
    }

    let threshold = k * 1.4826 * mad;

    values
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| {
            if (v - median).abs() > threshold {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometric_mean_simple() {
        let values = vec![1.0, 10.0, 100.0];
        let result = calculate_geometric_mean(&values).unwrap();
        // geomean([1, 10, 100]) = (1*10*100)^(1/3) = 1000^(1/3) = 10
        assert!((result - 10.0).abs() < 0.0001);
    }

    #[test]
    fn test_arithmetic_mean_simple() {
        let values = vec![2.0, 4.0, 6.0];
        let result = calculate_arithmetic_mean(&values).unwrap();
        assert!((result - 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_harmonic_mean_simple() {
        let values = vec![1.0, 2.0, 4.0];
        let result = calculate_harmonic_mean(&values).unwrap();
        // harmonic mean = 3 / (1/1 + 1/2 + 1/4) = 3 / 1.75 = 1.714
        assert!((result - 1.714).abs() < 0.01);
    }

    #[test]
    fn test_median_odd_count() {
        let values = vec![1.0, 3.0, 2.0];
        let result = calculate_median(&values).unwrap();
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_median_even_count() {
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let result = calculate_median(&values).unwrap();
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_mad_calculation() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let median = 3.0;
        let mad = calculate_mad(&values, median).unwrap();
        // deviations: [2, 1, 0, 1, 2], median = 1
        assert!((mad - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_outlier_detection_no_outliers() {
        let values = vec![10.0, 11.0, 10.5, 10.2, 10.8];
        let outliers = detect_outliers_mad(&values, 3.0);
        assert!(outliers.is_empty());
    }

    #[test]
    fn test_outlier_detection_with_outlier() {
        let values = vec![10.0, 11.0, 10.5, 10.2, 100.0];
        let outliers = detect_outliers_mad(&values, 3.0);
        assert!(!outliers.is_empty());
        assert!(outliers.contains(&4));
    }
}
