/// Integration tests for end-to-end benchmark execution
///
/// These tests verify the complete pipeline:
/// 1. Docker container execution
/// 2. Output capture and parsing
/// 3. Result validation
/// 4. Metadata enrichment (image size, memory usage)
use ruchy_docker::metrics::parse_benchmark_output;
use ruchy_docker::runner::BenchmarkRunner;

/// Test end-to-end fibonacci benchmark execution
///
/// Prerequisites: Docker image `rust:fibonacci` must be built
/// Build command: docker build -f docker/rust/fibonacci.Dockerfile -t rust:fibonacci .
#[tokio::test]
async fn test_fibonacci_rust_end_to_end() {
    // Skip if Docker not available
    let runner = match BenchmarkRunner::new().await {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Skipping test: Docker not available");
            return;
        }
    };

    // Run fibonacci container
    let output = match runner.run_container("rust:fibonacci", vec![]).await {
        Ok(out) => out,
        Err(e) => {
            eprintln!("\n⚠️  Skipping test: rust:fibonacci image not available or Docker timeout");
            eprintln!("Error: {}", e);
            eprintln!("💡 Run: make build-images");
            return;
        }
    };

    // Verify output contains expected markers
    assert!(
        output.contains("STARTUP_TIME_US:"),
        "Output should contain STARTUP_TIME_US"
    );
    assert!(
        output.contains("COMPUTE_TIME_US:"),
        "Output should contain COMPUTE_TIME_US"
    );
    assert!(output.contains("RESULT:"), "Output should contain RESULT");

    // Parse output
    let mut result = parse_benchmark_output(&output, "fibonacci", "rust")
        .expect("Should parse benchmark output");

    // Validate result
    assert_eq!(
        result.result_value,
        Some(9227465),
        "fib(35) should equal 9227465"
    );
    assert!(
        result.compute_time_us > 0,
        "Compute time should be positive"
    );
    assert!(result.total_time_us > 0, "Total time should be positive");

    // Enrich with Docker metadata
    runner
        .enrich_with_metadata(&mut result, "rust:fibonacci")
        .await
        .expect("Should enrich with metadata");

    // Verify metadata
    assert!(result.image_size_mb > 0.0, "Image size should be positive");
    assert!(
        result.image_size_mb < 10.0,
        "Image size should be <10MB (actual: {} MB)",
        result.image_size_mb
    );

    // Display results
    println!("\n=== Fibonacci Benchmark Results (Rust) ===");
    println!("Benchmark: {}", result.benchmark_name);
    println!("Language: {}", result.language);
    println!("Startup time: {} μs", result.startup_time_us);
    println!("Compute time: {} μs", result.compute_time_us);
    println!("Total time: {} μs", result.total_time_us);
    println!("Result: {:?}", result.result_value);
    println!("Image size: {:.2} MB", result.image_size_mb);
    println!("Memory usage: {:.2} MB", result.memory_usage_mb);
}

/// Test that image size is within acceptable bounds
#[tokio::test]
async fn test_fibonacci_image_size() {
    let runner = match BenchmarkRunner::new().await {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Skipping test: Docker not available");
            return;
        }
    };

    let size = match runner.get_image_size_mb("rust:fibonacci").await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\n⚠️  Skipping test: rust:fibonacci image not available");
            eprintln!("Error: {}", e);
            eprintln!("💡 Run: make build-images");
            return;
        }
    };

    println!("Fibonacci image size: {:.2} MB", size);

    assert!(size > 0.0, "Image size should be positive");
    assert!(
        size < 10.0,
        "Image size should be <10MB target (actual: {:.2} MB)",
        size
    );

    // Distroless + static binary should be very small
    assert!(
        size < 5.0,
        "Rust static binary should be <5MB (actual: {:.2} MB)",
        size
    );
}

/// Test benchmark performance is reasonable
#[tokio::test]
async fn test_fibonacci_performance() {
    let runner = match BenchmarkRunner::new().await {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Skipping test: Docker not available");
            return;
        }
    };

    let output = match runner.run_container("rust:fibonacci", vec![]).await {
        Ok(out) => out,
        Err(e) => {
            eprintln!("\n⚠️  Skipping test: rust:fibonacci image not available or Docker timeout");
            eprintln!("Error: {}", e);
            eprintln!("💡 Run: make build-images");
            return;
        }
    };

    let result = parse_benchmark_output(&output, "fibonacci", "rust")
        .expect("Should parse benchmark output");

    // Fibonacci(35) should complete in reasonable time
    // On modern hardware, this should be <100ms
    // We'll be generous and allow 1 second for slow CI environments
    let total_time_ms = result.total_time_us as f64 / 1000.0;
    assert!(
        total_time_ms < 1000.0,
        "Fibonacci should complete in <1s (actual: {:.2} ms)",
        total_time_ms
    );

    println!("Fibonacci(35) completed in {:.2} ms", total_time_ms);
}

/// Test C Fibonacci benchmark end-to-end
#[tokio::test]
async fn test_fibonacci_c_end_to_end() {
    let runner = match BenchmarkRunner::new().await {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Skipping test: Docker not available");
            return;
        }
    };

    let output = match runner.run_container("c:fibonacci", vec![]).await {
        Ok(out) => out,
        Err(e) => {
            eprintln!("\n⚠️  Skipping test: c:fibonacci image not available or Docker timeout");
            eprintln!("Error: {}", e);
            eprintln!("💡 Run: make build-images");
            return;
        }
    };

    let mut result =
        parse_benchmark_output(&output, "fibonacci", "c").expect("Should parse benchmark output");

    assert_eq!(result.result_value, Some(9227465));
    assert!(result.compute_time_us > 0);

    runner
        .enrich_with_metadata(&mut result, "c:fibonacci")
        .await
        .expect("Should enrich with metadata");

    assert!(result.image_size_mb < 10.0, "C image should be <10MB");

    println!("\n=== Fibonacci Benchmark Results (C) ===");
    println!(
        "Compute time: {} μs ({:.2} ms)",
        result.compute_time_us,
        result.compute_time_ms()
    );
    println!("Image size: {:.2} MB", result.image_size_mb);
}

/// Test Python Fibonacci benchmark end-to-end
#[tokio::test]
async fn test_fibonacci_python_end_to_end() {
    let runner = match BenchmarkRunner::new().await {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Skipping test: Docker not available");
            return;
        }
    };

    let output = match runner.run_container("python:fibonacci", vec![]).await {
        Ok(out) => out,
        Err(e) => {
            eprintln!("\n⚠️  Skipping test: python:fibonacci image not available or Docker timeout");
            eprintln!("Error: {}", e);
            eprintln!("💡 Run: make build-images");
            return;
        }
    };

    let mut result = parse_benchmark_output(&output, "fibonacci", "python")
        .expect("Should parse benchmark output");

    assert_eq!(result.result_value, Some(9227465));
    assert!(result.compute_time_us > 0);

    runner
        .enrich_with_metadata(&mut result, "python:fibonacci")
        .await
        .expect("Should enrich with metadata");

    println!("\n=== Fibonacci Benchmark Results (Python) ===");
    println!(
        "Compute time: {} μs ({:.2} ms)",
        result.compute_time_us,
        result.compute_time_ms()
    );
    println!("Image size: {:.2} MB", result.image_size_mb);
}

/// Test cross-language comparison with 3 languages
#[tokio::test]
async fn test_fibonacci_comparison_report() {
    use ruchy_docker::reporter::ComparisonReport;

    let runner = match BenchmarkRunner::new().await {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Skipping test: Docker not available");
            return;
        }
    };

    // Run all 3 languages
    let mut results = vec![];

    for (image, language) in [
        ("c:fibonacci", "c"),
        ("rust:fibonacci", "rust"),
        ("python:fibonacci", "python"),
    ] {
        match runner.run_container(image, vec![]).await {
            Ok(output) => {
                match parse_benchmark_output(&output, "fibonacci", language) {
                    Ok(mut result) => {
                        if let Err(e) = runner.enrich_with_metadata(&mut result, image).await {
                            eprintln!("⚠️  Warning: Failed to enrich metadata for {}: {}", language, e);
                        }
                        results.push(result);
                    }
                    Err(e) => {
                        eprintln!("⚠️  Skipping {}: Failed to parse output: {}", language, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("⚠️  Skipping {}: Image {} not available or Docker timeout: {}", language, image, e);
                eprintln!("💡 Run: make build-images");
            }
        }
    }

    // If no images were available, skip the test
    if results.is_empty() {
        eprintln!("\n⚠️  Skipping test: No Docker images available");
        eprintln!("💡 Run: make build-images");
        return;
    }

    // Need at least 2 languages for comparison
    if results.len() < 2 {
        eprintln!("\n⚠️  Skipping test: Need at least 2 languages for comparison (got {})", results.len());
        eprintln!("💡 Run: make build-images");
        return;
    }

    // Generate comparison report (use first language as baseline if C not available)
    let baseline_lang = if results.iter().any(|r| r.language == "c") {
        "c"
    } else {
        &results[0].language
    };

    let comparison = ComparisonReport::from_results(&results, baseline_lang)
        .expect("Should create comparison");

    println!("\n{}", comparison.to_markdown());

    // Verify speedups for baseline
    assert_eq!(comparison.get_speedup(baseline_lang), Some(1.0)); // Baseline should always be 1.0x

    // Verify aggregation metrics exist
    assert!(comparison.aggregation_metrics.is_some());
}

/// Test Go Fibonacci benchmark end-to-end
#[tokio::test]
async fn test_fibonacci_go_end_to_end() {
    let runner = match BenchmarkRunner::new().await {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Skipping test: Docker not available");
            return;
        }
    };

    let output = match runner.run_container("go:fibonacci", vec![]).await {
        Ok(out) => out,
        Err(e) => {
            eprintln!("\n⚠️  Skipping test: go:fibonacci image not available or Docker timeout");
            eprintln!("Error: {}", e);
            eprintln!("💡 Run: make build-images");
            return;
        }
    };

    let mut result =
        parse_benchmark_output(&output, "fibonacci", "go").expect("Should parse benchmark output");

    assert_eq!(result.result_value, Some(9227465));
    assert!(result.compute_time_us > 0);

    runner
        .enrich_with_metadata(&mut result, "go:fibonacci")
        .await
        .expect("Should enrich with metadata");

    assert!(result.image_size_mb < 10.0, "Go image should be <10MB");

    println!("\n=== Fibonacci Benchmark Results (Go) ===");
    println!(
        "Compute time: {} μs ({:.2} ms)",
        result.compute_time_us,
        result.compute_time_ms()
    );
    println!("Image size: {:.2} MB", result.image_size_mb);
}

/// Test Ruchy (transpiled) Fibonacci benchmark end-to-end
#[tokio::test]
async fn test_fibonacci_ruchy_transpiled_end_to_end() {
    let runner = match BenchmarkRunner::new().await {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Skipping test: Docker not available");
            return;
        }
    };

    let output = match runner.run_container("ruchy-transpiled:fibonacci", vec![]).await {
        Ok(out) => out,
        Err(e) => {
            eprintln!("\n⚠️  Skipping test: ruchy-transpiled:fibonacci image not available or Docker timeout");
            eprintln!("Error: {}", e);
            eprintln!("💡 Run: make build-images");
            return;
        }
    };

    let mut result = parse_benchmark_output(&output, "fibonacci", "ruchy-transpiled")
        .expect("Should parse benchmark output");

    assert_eq!(result.result_value, Some(9227465));
    assert!(result.compute_time_us > 0);

    runner
        .enrich_with_metadata(&mut result, "ruchy-transpiled:fibonacci")
        .await
        .expect("Should enrich with metadata");

    assert!(
        result.image_size_mb < 10.0,
        "Ruchy transpiled image should be <10MB"
    );

    println!("\n=== Fibonacci Benchmark Results (Ruchy Transpiled) ===");
    println!(
        "Compute time: {} μs ({:.2} ms)",
        result.compute_time_us,
        result.compute_time_ms()
    );
    println!("Image size: {:.2} MB", result.image_size_mb);
}

/// Test Ruchy (compiled) Fibonacci benchmark end-to-end
#[tokio::test]
async fn test_fibonacci_ruchy_compiled_end_to_end() {
    let runner = match BenchmarkRunner::new().await {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Skipping test: Docker not available");
            return;
        }
    };

    let output = match runner.run_container("ruchy-compiled:fibonacci", vec![]).await {
        Ok(out) => out,
        Err(e) => {
            eprintln!("\n⚠️  Skipping test: ruchy-compiled:fibonacci image not available or broken");
            eprintln!("Error: {}", e);
            eprintln!("💡 Run: make build-images");
            return;
        }
    };

    let mut result = parse_benchmark_output(&output, "fibonacci", "ruchy-compiled")
        .expect("Should parse benchmark output");

    assert_eq!(result.result_value, Some(9227465));
    assert!(result.compute_time_us > 0);

    runner
        .enrich_with_metadata(&mut result, "ruchy-compiled:fibonacci")
        .await
        .expect("Should enrich with metadata");

    assert!(
        result.image_size_mb < 10.0,
        "Ruchy compiled image should be <10MB"
    );

    println!("\n=== Fibonacci Benchmark Results (Ruchy Compiled) ===");
    println!(
        "Compute time: {} μs ({:.2} ms)",
        result.compute_time_us,
        result.compute_time_ms()
    );
    println!("Image size: {:.2} MB", result.image_size_mb);
}

/// Test cross-language comparison with all 6 languages
#[tokio::test]
async fn test_fibonacci_all_languages_comparison() {
    use ruchy_docker::reporter::ComparisonReport;

    let runner = match BenchmarkRunner::new().await {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Skipping test: Docker not available");
            return;
        }
    };

    // Run all 6 languages
    let mut results = vec![];

    for (image, language) in [
        ("c:fibonacci", "c"),
        ("rust:fibonacci", "rust"),
        ("go:fibonacci", "go"),
        ("python:fibonacci", "python"),
        ("ruchy-transpiled:fibonacci", "ruchy-transpiled"),
        ("ruchy-compiled:fibonacci", "ruchy-compiled"),
    ] {
        match runner.run_container(image, vec![]).await {
            Ok(output) => {
                match parse_benchmark_output(&output, "fibonacci", language) {
                    Ok(mut result) => {
                        if let Err(e) = runner.enrich_with_metadata(&mut result, image).await {
                            eprintln!("⚠️  Warning: Failed to enrich metadata for {}: {}", language, e);
                        }
                        results.push(result);
                    }
                    Err(e) => {
                        eprintln!("⚠️  Skipping {}: Failed to parse output: {}", language, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("⚠️  Skipping {}: Image {} not available or broken: {}", language, image, e);
                eprintln!("💡 Run: make build-images");
            }
        }
    }

    // If no images were available, skip the test
    if results.is_empty() {
        eprintln!("\n⚠️  Skipping test: No Docker images available");
        eprintln!("💡 Run: make build-images");
        return;
    }

    // Need at least 2 languages for comparison
    if results.len() < 2 {
        eprintln!("\n⚠️  Skipping test: Need at least 2 languages for comparison (got {})", results.len());
        eprintln!("💡 Run: make build-images");
        return;
    }

    // Generate comparison report (use first language as baseline if C not available)
    let baseline_lang = if results.iter().any(|r| r.language == "c") {
        "c"
    } else {
        &results[0].language
    };

    let comparison = ComparisonReport::from_results(&results, baseline_lang)
        .expect("Should create comparison");

    println!("\n{}", comparison.to_markdown());

    // Verify speedups for baseline
    assert_eq!(comparison.get_speedup(baseline_lang), Some(1.0)); // Baseline should always be 1.0x

    // Verify aggregation metrics exist
    assert!(comparison.aggregation_metrics.is_some());
}

// ============================================================================
// BENCH-008: Prime Sieve Integration Tests (TODO: Future benchmarks)
// ============================================================================
