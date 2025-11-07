use anyhow::Result;
use std::collections::HashMap;

use crate::analyzer::AggregationMetrics;
use crate::metrics::BenchmarkResult;

/// Results reporting and visualization
///
/// This module generates benchmark reports in multiple formats:
/// - JSON: Machine-readable, structured data for programmatic analysis
/// - Markdown: Human-readable tables for README files and documentation
/// - Comparison reports: Speedup calculations relative to baseline language
///
/// Generate JSON report from benchmark results
///
/// # Arguments
/// * `results` - Slice of BenchmarkResult structs
///
/// # Returns
/// * `Ok(String)` - JSON-formatted report
/// * `Err(_)` - Serialization error
pub fn generate_json_report(results: &[BenchmarkResult]) -> Result<String> {
    let json = serde_json::to_string_pretty(results)?;
    Ok(json)
}

/// Generate Markdown table from benchmark results
///
/// Creates a formatted table with columns:
/// - Language
/// - Startup Time (ms)
/// - Compute Time (ms)
/// - Total Time (ms)
/// - Image Size (MB)
/// - Memory Usage (MB)
///
/// # Arguments
/// * `results` - Slice of BenchmarkResult structs
/// * `benchmark_name` - Name of the benchmark for the header
///
/// # Returns
/// * Markdown-formatted table as String
pub fn generate_markdown_table(results: &[BenchmarkResult], benchmark_name: &str) -> String {
    if results.is_empty() {
        return format!("# {} Benchmark\n\nNo results available.\n", benchmark_name);
    }

    let mut markdown = String::new();

    // Header
    markdown.push_str(&format!(
        "# {} Benchmark\n\n",
        capitalize_first(benchmark_name)
    ));

    // Table header
    markdown.push_str(
        "| Language | Startup (ms) | Compute (ms) | Total (ms) | Image Size (MB) | Memory (MB) |\n",
    );
    markdown.push_str(
        "|----------|--------------|--------------|------------|-----------------|-------------|\n",
    );

    // Table rows
    for result in results {
        markdown.push_str(&format!(
            "| {} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |\n",
            result.language,
            result.startup_time_ms(),
            result.compute_time_ms(),
            result.total_time_ms(),
            result.image_size_mb,
            result.memory_usage_mb
        ));
    }

    markdown.push('\n');
    markdown
}

/// Comparison report with speedup calculations
///
/// Compares all languages against a baseline language and calculates
/// speedup factors. Includes aggregation metrics (geometric, arithmetic,
/// harmonic means) for the speedup factors.
#[derive(Debug, Clone)]
pub struct ComparisonReport {
    /// Benchmark name
    pub benchmark_name: String,

    /// Baseline language for comparison
    pub baseline_language: String,

    /// Map of language -> speedup factor (baseline_time / language_time)
    /// Speedup > 1.0 means language is faster than baseline
    /// Speedup < 1.0 means language is slower than baseline
    pub speedups: HashMap<String, f64>,

    /// Original benchmark results
    pub results: Vec<BenchmarkResult>,

    /// Aggregation metrics for speedup factors
    pub aggregation_metrics: Option<AggregationMetrics>,
}

impl ComparisonReport {
    /// Create comparison report from benchmark results
    ///
    /// # Arguments
    /// * `results` - Slice of BenchmarkResult structs (must all be same benchmark)
    /// * `baseline_language` - Language to use as baseline (e.g., "rust")
    ///
    /// # Returns
    /// * `Ok(ComparisonReport)` - Comparison report with speedups
    /// * `Err(_)` - Error if baseline not found or results are empty
    pub fn from_results(results: &[BenchmarkResult], baseline_language: &str) -> Result<Self> {
        if results.is_empty() {
            return Err(anyhow::anyhow!(
                "Cannot create comparison from empty results"
            ));
        }

        // Find baseline result
        let baseline = results
            .iter()
            .find(|r| r.language == baseline_language)
            .ok_or_else(|| {
                anyhow::anyhow!("Baseline language '{}' not found", baseline_language)
            })?;

        let baseline_time = baseline.total_time_us as f64;
        let benchmark_name = results[0].benchmark_name.clone();

        // Calculate speedups: baseline_time / language_time
        // Speedup > 1.0 means language is slower than baseline (takes more time)
        // Speedup < 1.0 means language is faster than baseline (takes less time)
        let mut speedups = HashMap::new();
        for result in results {
            let language_time = result.total_time_us as f64;
            let speedup = baseline_time / language_time;
            speedups.insert(result.language.clone(), speedup);
        }

        // Calculate aggregation metrics for speedup factors
        let speedup_values: Vec<f64> = speedups.values().copied().collect();
        let aggregation_metrics = AggregationMetrics::from_values(&speedup_values).ok();

        Ok(Self {
            benchmark_name,
            baseline_language: baseline_language.to_string(),
            speedups,
            results: results.to_vec(),
            aggregation_metrics,
        })
    }

    /// Get speedup factor for a specific language
    ///
    /// # Arguments
    /// * `language` - Language identifier
    ///
    /// # Returns
    /// * `Some(f64)` - Speedup factor
    /// * `None` - Language not found
    pub fn get_speedup(&self, language: &str) -> Option<f64> {
        self.speedups.get(language).copied()
    }

    /// Generate Markdown table with speedup calculations
    ///
    /// # Returns
    /// * Markdown-formatted comparison table as String
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();

        // Header
        markdown.push_str(&format!(
            "# {} Benchmark - Comparison (baseline: {})\n\n",
            capitalize_first(&self.benchmark_name),
            self.baseline_language
        ));

        // Table header
        markdown.push_str("| Language | Total Time (ms) | Speedup | Image Size (MB) |\n");
        markdown.push_str("|----------|-----------------|---------|----------------|\n");

        // Sort results by total time (fastest first)
        let mut sorted_results = self.results.clone();
        sorted_results.sort_by(|a, b| a.total_time_us.cmp(&b.total_time_us));

        // Table rows
        for result in &sorted_results {
            let speedup = self.speedups.get(&result.language).unwrap_or(&1.0);
            markdown.push_str(&format!(
                "| {} | {:.2} | {:.2}x | {:.2} |\n",
                result.language,
                result.total_time_ms(),
                speedup,
                result.image_size_mb
            ));
        }

        // Aggregation metrics
        if let Some(ref metrics) = self.aggregation_metrics {
            markdown.push_str("\n## Aggregation Metrics\n\n");
            markdown.push_str(&format!(
                "- **Geometric Mean Speedup**: {:.2}x\n",
                metrics.geometric_mean
            ));
            markdown.push_str(&format!(
                "- **Arithmetic Mean Speedup**: {:.2}x\n",
                metrics.arithmetic_mean
            ));
            markdown.push_str(&format!(
                "- **Harmonic Mean Speedup**: {:.2}x\n",
                metrics.harmonic_mean
            ));

            if !metrics.outlier_indices.is_empty() {
                markdown.push_str(&format!(
                    "\n⚠️  **Outliers detected**: {} measurement(s)\n",
                    metrics.outlier_indices.len()
                ));
            }
        }

        markdown.push('\n');
        markdown
    }
}

/// Capitalize first letter of string
fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first("fibonacci"), "Fibonacci");
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_json_report_empty() {
        let results: Vec<BenchmarkResult> = vec![];
        let json = generate_json_report(&results).unwrap();
        assert_eq!(json, "[]");
    }

    #[test]
    fn test_markdown_table_empty() {
        let results: Vec<BenchmarkResult> = vec![];
        let markdown = generate_markdown_table(&results, "test");
        assert!(markdown.contains("No results"));
    }

    #[test]
    fn test_speedup_calculation() {
        // Baseline: 100ms
        // Other: 200ms
        // Speedup: 100/200 = 0.5 (other is 2x slower, or 0.5x as fast)
        let results = vec![
            BenchmarkResult {
                benchmark_name: "test".to_string(),
                language: "baseline".to_string(),
                startup_time_us: 0,
                compute_time_us: 100000,
                total_time_us: 100000,
                result_value: None,
                image_size_mb: 1.0,
                memory_usage_mb: 1.0,
            },
            BenchmarkResult {
                benchmark_name: "test".to_string(),
                language: "slow".to_string(),
                startup_time_us: 0,
                compute_time_us: 200000,
                total_time_us: 200000,
                result_value: None,
                image_size_mb: 1.0,
                memory_usage_mb: 1.0,
            },
        ];

        let comparison = ComparisonReport::from_results(&results, "baseline").unwrap();

        assert_eq!(comparison.get_speedup("baseline"), Some(1.0));
        assert_eq!(comparison.get_speedup("slow"), Some(0.5));
    }
}
