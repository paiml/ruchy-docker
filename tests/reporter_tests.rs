/// RED Phase: Tests for benchmark reporter
///
/// These tests validate report generation in multiple formats:
/// - JSON: Machine-readable, structured data
/// - Markdown: Human-readable tables with GitHub/GitLab compatibility
/// - Comparison reports: Speedup calculations relative to baseline
use ruchy_docker::metrics::BenchmarkResult;
use ruchy_docker::reporter::{generate_json_report, generate_markdown_table, ComparisonReport};

/// Test JSON report generation for single benchmark result
#[test]
fn test_json_report_single_result() {
    let result = BenchmarkResult {
        benchmark_name: "fibonacci".to_string(),
        language: "rust".to_string(),
        startup_time_us: 100,
        compute_time_us: 20000,
        total_time_us: 20100,
        result_value: Some(9227465),
        image_size_mb: 2.51,
        memory_usage_mb: 8.5,
    };

    let json = generate_json_report(&[result]).expect("Should generate JSON");

    // Verify JSON is valid and contains expected fields
    assert!(json.contains("\"benchmark_name\""));
    assert!(json.contains("\"fibonacci\""));
    assert!(json.contains("\"rust\""));
    assert!(json.contains("\"total_time_us\": 20100")); // Pretty-printed has spaces
    assert!(json.contains("\"image_size_mb\": 2.51"));
}

/// Test JSON report with multiple results
#[test]
fn test_json_report_multiple_results() {
    let results = vec![
        BenchmarkResult {
            benchmark_name: "fibonacci".to_string(),
            language: "rust".to_string(),
            startup_time_us: 100,
            compute_time_us: 20000,
            total_time_us: 20100,
            result_value: Some(9227465),
            image_size_mb: 2.51,
            memory_usage_mb: 8.5,
        },
        BenchmarkResult {
            benchmark_name: "fibonacci".to_string(),
            language: "python".to_string(),
            startup_time_us: 50000,
            compute_time_us: 800000,
            total_time_us: 850000,
            result_value: Some(9227465),
            image_size_mb: 52.3,
            memory_usage_mb: 45.6,
        },
    ];

    let json = generate_json_report(&results).expect("Should generate JSON");

    // Verify both results are in the JSON
    assert!(json.contains("\"rust\""));
    assert!(json.contains("\"python\""));

    // Parse JSON to verify it's valid
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Should be valid JSON");
    assert!(parsed.is_array());
    assert_eq!(parsed.as_array().unwrap().len(), 2);
}

/// Test Markdown table generation
#[test]
fn test_markdown_table_generation() {
    let results = vec![
        BenchmarkResult {
            benchmark_name: "fibonacci".to_string(),
            language: "rust".to_string(),
            startup_time_us: 100,
            compute_time_us: 19400,
            total_time_us: 19500,
            result_value: Some(9227465),
            image_size_mb: 2.51,
            memory_usage_mb: 8.5,
        },
        BenchmarkResult {
            benchmark_name: "fibonacci".to_string(),
            language: "python".to_string(),
            startup_time_us: 50000,
            compute_time_us: 800000,
            total_time_us: 850000,
            result_value: Some(9227465),
            image_size_mb: 52.3,
            memory_usage_mb: 45.6,
        },
    ];

    let markdown = generate_markdown_table(&results, "fibonacci");

    // Verify Markdown table structure
    assert!(markdown.contains("# Fibonacci Benchmark"));
    assert!(markdown.contains("| Language"));
    assert!(markdown.contains("| rust"));
    assert!(markdown.contains("| python"));
    assert!(markdown.contains("19.50")); // Total time in ms
    assert!(markdown.contains("850.00")); // Python time in ms
}

/// Test Markdown table formatting with proper alignment
#[test]
fn test_markdown_table_formatting() {
    let results = vec![BenchmarkResult {
        benchmark_name: "fibonacci".to_string(),
        language: "rust".to_string(),
        startup_time_us: 100,
        compute_time_us: 20000,
        total_time_us: 20100,
        result_value: Some(9227465),
        image_size_mb: 2.51,
        memory_usage_mb: 8.5,
    }];

    let markdown = generate_markdown_table(&results, "fibonacci");

    // Verify table has header separator
    assert!(markdown.contains("|---"));
    // Verify time conversions to milliseconds
    assert!(markdown.contains("ms"));
    assert!(markdown.contains("MB"));
}

/// Test comparison report with speedup calculations
#[test]
fn test_comparison_report_speedups() {
    let results = vec![
        BenchmarkResult {
            benchmark_name: "fibonacci".to_string(),
            language: "rust".to_string(),
            startup_time_us: 0,
            compute_time_us: 20000,
            total_time_us: 20000,
            result_value: Some(9227465),
            image_size_mb: 2.51,
            memory_usage_mb: 8.5,
        },
        BenchmarkResult {
            benchmark_name: "fibonacci".to_string(),
            language: "python".to_string(),
            startup_time_us: 50000,
            compute_time_us: 800000,
            total_time_us: 850000,
            result_value: Some(9227465),
            image_size_mb: 52.3,
            memory_usage_mb: 45.6,
        },
    ];

    let comparison =
        ComparisonReport::from_results(&results, "rust").expect("Should create comparison");

    // Python is ~42.5x slower than Rust (850ms / 20ms)
    // Speedup = baseline_time / language_time = 20000 / 850000 = 0.0235
    let python_speedup = comparison
        .get_speedup("python")
        .expect("Should have speedup");
    assert!(
        (python_speedup - 0.0235).abs() < 0.01,
        "Python speedup should be ~0.024 (42.5x slower)"
    );

    // Rust baseline should have speedup of 1.0
    let rust_speedup = comparison.get_speedup("rust").expect("Should have speedup");
    assert!((rust_speedup - 1.0).abs() < 0.01, "Baseline should be 1.0x");
}

/// Test comparison report Markdown output
#[test]
fn test_comparison_report_markdown() {
    let results = vec![
        BenchmarkResult {
            benchmark_name: "fibonacci".to_string(),
            language: "rust".to_string(),
            startup_time_us: 0,
            compute_time_us: 20000,
            total_time_us: 20000,
            result_value: Some(9227465),
            image_size_mb: 2.51,
            memory_usage_mb: 8.5,
        },
        BenchmarkResult {
            benchmark_name: "fibonacci".to_string(),
            language: "c".to_string(),
            startup_time_us: 0,
            compute_time_us: 18000,
            total_time_us: 18000,
            result_value: Some(9227465),
            image_size_mb: 2.1,
            memory_usage_mb: 5.2,
        },
    ];

    let comparison =
        ComparisonReport::from_results(&results, "rust").expect("Should create comparison");
    let markdown = comparison.to_markdown();

    // Verify speedup column
    assert!(markdown.contains("Speedup"));
    assert!(markdown.contains("1.00x")); // Rust baseline
    assert!(markdown.contains("1.11x")); // C is faster (speedup = 20000/18000 = 1.11)
}

/// Test aggregation metrics in comparison report
#[test]
fn test_comparison_with_aggregation_metrics() {
    let results = vec![
        BenchmarkResult {
            benchmark_name: "fibonacci".to_string(),
            language: "rust".to_string(),
            startup_time_us: 0,
            compute_time_us: 20000,
            total_time_us: 20000,
            result_value: Some(9227465),
            image_size_mb: 2.51,
            memory_usage_mb: 8.5,
        },
        BenchmarkResult {
            benchmark_name: "fibonacci".to_string(),
            language: "go".to_string(),
            startup_time_us: 5000,
            compute_time_us: 40000,
            total_time_us: 45000,
            result_value: Some(9227465),
            image_size_mb: 6.2,
            memory_usage_mb: 12.3,
        },
        BenchmarkResult {
            benchmark_name: "fibonacci".to_string(),
            language: "python".to_string(),
            startup_time_us: 50000,
            compute_time_us: 800000,
            total_time_us: 850000,
            result_value: Some(9227465),
            image_size_mb: 52.3,
            memory_usage_mb: 45.6,
        },
    ];

    let comparison =
        ComparisonReport::from_results(&results, "rust").expect("Should create comparison");

    // Should have aggregation metrics
    assert!(comparison.aggregation_metrics.is_some());
    let metrics = comparison.aggregation_metrics.unwrap();

    // Speedups: 1.0 (Rust), 0.444 (Go is 2.25x slower), 0.0235 (Python is 42.5x slower)
    // Geometric mean should be < 1.0 since most are slower
    assert!(
        metrics.geometric_mean < 1.0,
        "Geometric mean speedup should be < 1.0 (most languages slower)"
    );
}

/// Test empty results handling
#[test]
fn test_empty_results() {
    let results: Vec<BenchmarkResult> = vec![];

    let json = generate_json_report(&results);
    assert!(json.is_ok());
    assert_eq!(json.unwrap(), "[]");

    let markdown = generate_markdown_table(&results, "test");
    assert!(markdown.contains("No results"));
}
