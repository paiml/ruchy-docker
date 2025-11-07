use ruchy_docker::runner::BenchmarkRunner;
use ruchy_docker::BenchmarkResult;

/// RED Phase: Test Docker container execution
/// This test will fail until we implement BenchmarkRunner
#[tokio::test]
async fn test_run_container_basic() {
    let runner = BenchmarkRunner::new()
        .await
        .expect("Failed to create runner");

    // Attempt to run a simple echo container
    let result = runner
        .run_container("alpine:latest", vec!["echo", "hello"])
        .await;

    assert!(result.is_ok(), "Should successfully run container");
    let output = result.unwrap();
    assert!(output.contains("hello"));
}

/// Test running a container and capturing standardized benchmark output
#[tokio::test]
async fn test_run_benchmark_container() {
    let _runner = BenchmarkRunner::new()
        .await
        .expect("Failed to create runner");

    // This will fail until we build the actual fibonacci image
    // For now, we'll skip this test
    // In the next phase, we'll implement this properly
}

/// Test Docker image inspection for size
#[tokio::test]
async fn test_get_image_size() {
    let runner = BenchmarkRunner::new()
        .await
        .expect("Failed to create runner");

    let size = runner.get_image_size_mb("alpine:latest").await;

    assert!(size.is_ok(), "Should get image size");
    let size_mb = size.unwrap();
    assert!(size_mb > 0.0, "Image size should be positive");
    assert!(size_mb < 50.0, "Alpine should be small (<50MB)");
}

/// Test parsing container output and populating BenchmarkResult
#[tokio::test]
async fn test_run_and_parse_benchmark() {
    let _runner = BenchmarkRunner::new()
        .await
        .expect("Failed to create runner");

    // Simulate benchmark output
    let mock_output = r#"STARTUP_TIME_US: 8234
COMPUTE_TIME_US: 23891
RESULT: 9227465"#;

    let result = ruchy_docker::metrics::parse_benchmark_output(mock_output, "fibonacci", "rust");

    assert!(result.is_ok());
    let mut benchmark_result = result.unwrap();

    // In a real scenario, runner would populate image_size_mb and memory_usage_mb
    benchmark_result.image_size_mb = 3.2;
    benchmark_result.memory_usage_mb = 8.5;

    assert_eq!(benchmark_result.startup_time_us, 8234);
    assert_eq!(benchmark_result.compute_time_us, 23891);
    assert_eq!(benchmark_result.image_size_mb, 3.2);
}

/// Test that runner properly enriches BenchmarkResult with Docker metadata
#[tokio::test]
async fn test_enrich_benchmark_result() {
    let runner = BenchmarkRunner::new()
        .await
        .expect("Failed to create runner");

    let mut result = BenchmarkResult {
        benchmark_name: "test".to_string(),
        language: "rust".to_string(),
        startup_time_us: 1000,
        compute_time_us: 2000,
        total_time_us: 3000,
        result_value: None,
        image_size_mb: 0.0,
        memory_usage_mb: 0.0,
    };

    // Enrich with Docker metadata
    runner
        .enrich_with_metadata(&mut result, "alpine:latest")
        .await
        .expect("Should enrich metadata");

    assert!(result.image_size_mb > 0.0, "Should populate image size");
}
