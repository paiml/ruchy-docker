/**
 * Proportional ASCII chart generation library
 *
 * This module provides functions to generate mathematically accurate
 * ASCII bar charts where bar lengths are proportional to values.
 */

export interface PerformanceData {
  name: string;
  time: number;
}

export interface SizeData {
  name: string;
  value: number;
  unit: string;
}

/**
 * Generate a proportional ASCII performance chart.
 */
export function generatePerformanceChart(
  data: PerformanceData[],
  maxWidth: number = 70,
): string {
  if (data.length === 0) return "";

  // Find max value to scale proportionally
  const maxValue = Math.max(...data.map((d) => d.time));

  // Calculate scale factor: characters per unit
  const scale = maxWidth / maxValue;

  const lines: string[] = [];

  for (const { name, time } of data) {
    // Calculate bar length proportionally
    const barLength = Math.floor(time * scale);
    const bar = "█".repeat(barLength);

    // Format the line with proper spacing
    const line = `${name.padEnd(12)}${bar} ${time.toFixed(2)}ms`;
    lines.push(line);
  }

  // Add scale ruler
  const rulerIndent = " ".repeat(12);
  const tickInterval = 50; // ms
  const numTicks = Math.floor(maxValue / tickInterval) + 1;

  // Create ruler with tick marks
  const rulerChars: string[] = new Array(maxWidth).fill("-");
  const tickLabels: number[] = [];

  for (let i = 0; i <= numTicks; i++) {
    const tickValue = i * tickInterval;
    const tickPos = Math.floor(tickValue * scale);
    tickLabels.push(tickValue);

    if (tickPos < maxWidth) {
      rulerChars[tickPos] = i % 2 === 0 ? "|" : "-";
    }
  }

  const rulerLine = rulerIndent + rulerChars.join("");
  lines.push(rulerLine);

  // Add scale labels
  let scaleLine = rulerIndent;
  for (let i = 0; i < tickLabels.length; i++) {
    const tickValue = tickLabels[i];
    const tickPos = Math.floor(tickValue * scale);

    if (tickPos < maxWidth - 3 && i % 2 === 0) {
      const label = tickValue.toString();
      const padding = tickPos - (scaleLine.length - 12) - Math.floor(
        label.length / 2,
      );

      if (padding > 0) {
        scaleLine += " ".repeat(padding) + label;
      }
    }
  }
  scaleLine += "ms";
  lines.push(scaleLine);

  return lines.join("\n");
}

/**
 * Generate a proportional ASCII size chart.
 */
export function generateSizeChart(
  data: SizeData[],
  maxWidth: number = 70,
): string {
  if (data.length === 0) return "";

  // Convert all to bytes for fair comparison
  function toBytes(value: number, unit: string): number {
    const upperUnit = unit.toUpperCase();
    if (upperUnit === "KB") return value * 1024;
    if (upperUnit === "MB") return value * 1024 * 1024;
    if (upperUnit === "GB") return value * 1024 * 1024 * 1024;
    return value;
  }

  // Convert all values to bytes
  const dataBytes = data.map((d) => ({
    name: d.name,
    bytes: toBytes(d.value, d.unit),
    unit: d.unit,
    originalValue: d.value,
  }));

  // Find max value
  const maxValueBytes = Math.max(...dataBytes.map((d) => d.bytes));

  // Calculate scale
  const scale = maxWidth / maxValueBytes;

  const lines: string[] = [];

  for (const { name, bytes, unit, originalValue } of dataBytes) {
    let barLength = Math.floor(bytes * scale);

    // Minimum one char for visibility
    if (barLength === 0) barLength = 1;

    const bar = "█".repeat(barLength);

    // Format with original units for readability
    const line = `${name.padEnd(12)}${bar} ${Math.floor(originalValue)} ${unit}`;
    lines.push(line);
  }

  // Add ruler
  const rulerLine = " ".repeat(12) + "-".repeat(maxWidth);
  lines.push(rulerLine);

  // Add scale (show in MB for readability)
  const maxMb = maxValueBytes / (1024 * 1024);
  const intervals = [0, maxMb * 0.25, maxMb * 0.5, maxMb * 0.75, maxMb];

  let scaleLine = " ".repeat(12);
  for (const valMb of intervals) {
    const pos = Math.floor((valMb * 1024 * 1024) * scale);
    const padding = pos - (scaleLine.length - 12);

    if (padding > 0 && pos < maxWidth) {
      scaleLine += " ".repeat(padding) + Math.floor(valMb).toString();
    }
  }
  scaleLine += " MB";
  lines.push(scaleLine);

  return lines.join("\n");
}
