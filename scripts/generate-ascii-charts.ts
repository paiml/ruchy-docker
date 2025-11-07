#!/usr/bin/env -S deno run --allow-read
/**
 * Generate proportional ASCII charts for benchmark results.
 *
 * This script outputs charts to stdout for viewing or copying.
 */

import {
  generatePerformanceChart,
  generateSizeChart,
  type PerformanceData,
  type SizeData,
} from "./chart-generator.ts";

function main(): void {
  // Fibonacci performance data (CLI invocation times)
  const performanceData: PerformanceData[] = [
    { name: "C", time: 10.77 },
    { name: "Rust", time: 21.81 },
    { name: "Ruchy (C)", time: 22.47 },
    { name: "Ruchy (T)", time: 23.68 },
    { name: "Go", time: 38.04 },
    { name: "Deno", time: 70.11 },
    { name: "Julia 🧪", time: 252.91 },
    { name: "Python", time: 697.49 },
  ];

  // Binary size data (Docker images)
  const sizeData: SizeData[] = [
    { name: "Ruchy (T)", value: 312, unit: "KB" },
    { name: "Ruchy (C)", value: 314, unit: "KB" },
    { name: "Rust", value: 424, unit: "KB" },
    { name: "C", value: 695, unit: "KB" },
    { name: "Go", value: 1.41, unit: "MB" },
    { name: "Python", value: 119, unit: "MB" },
    { name: "Deno", value: 256, unit: "MB" },
    { name: "Julia 🧪", value: 711, unit: "MB" },
  ];

  console.log("**Performance Visualization** (Execution Time):");
  console.log();
  console.log("```");
  console.log(generatePerformanceChart(performanceData, 70));
  console.log("```");
  console.log();

  console.log("**Binary Size Comparison** (Docker images):");
  console.log();
  console.log("```");
  console.log(generateSizeChart(sizeData, 70));
  console.log("```");
}

if (import.meta.main) {
  main();
}
