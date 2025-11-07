#!/usr/bin/env -S deno run --allow-read --allow-write
/**
 * Safely update auto-generated chart sections in README.md
 *
 * This script:
 * 1. Reads README.md
 * 2. Finds sections marked with <!-- AUTO-GENERATED-CHART: ... -->
 * 3. Replaces content between start/end markers
 * 4. Creates backup before modification
 * 5. Validates the replacement was successful
 */

import { generatePerformanceChart, generateSizeChart } from "./chart-generator.ts";

interface ChartSection {
  name: string;
  startMarker: string;
  endMarker: string;
  generator: () => string;
}

/**
 * Safely replace a section in content between markers
 */
function replaceSectionBetweenMarkers(
  content: string,
  startMarker: string,
  endMarker: string,
  newContent: string,
): { success: boolean; content: string; error?: string } {
  // Find start marker
  const startIndex = content.indexOf(startMarker);
  if (startIndex === -1) {
    return {
      success: false,
      content,
      error: `Start marker not found: ${startMarker}`,
    };
  }

  // Find end marker
  const endIndex = content.indexOf(endMarker, startIndex);
  if (endIndex === -1) {
    return {
      success: false,
      content,
      error: `End marker not found: ${endMarker}`,
    };
  }

  // Find the end of the start marker line
  const startLineEnd = content.indexOf("\n", startIndex);
  if (startLineEnd === -1 || startLineEnd > endIndex) {
    return {
      success: false,
      content,
      error: "Invalid marker structure",
    };
  }

  // Find the start of the end marker line
  const endLineStart = content.lastIndexOf("\n", endIndex);
  if (endLineStart === -1 || endLineStart < startIndex) {
    return {
      success: false,
      content,
      error: "Invalid marker structure",
    };
  }

  // Replace content between markers
  const before = content.substring(0, startLineEnd + 1);
  const after = content.substring(endLineStart + 1);
  const newFullContent = before + newContent + "\n" + after;

  return { success: true, content: newFullContent };
}

/**
 * Create backup of file
 */
async function createBackup(filePath: string): Promise<void> {
  const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
  const backupPath = `${filePath}.backup-${timestamp}`;

  const content = await Deno.readTextFile(filePath);
  await Deno.writeTextFile(backupPath, content);

  console.log(`✅ Backup created: ${backupPath}`);
}

/**
 * Main update function
 */
async function updateReadmeCharts(readmePath: string): Promise<void> {
  console.log("🔧 Updating README.md charts...\n");

  // Define chart sections to update
  const sections: ChartSection[] = [
    {
      name: "Performance Chart",
      startMarker: "<!-- AUTO-GENERATED-CHART: performance-start -->",
      endMarker: "<!-- AUTO-GENERATED-CHART: performance-end -->",
      generator: () => {
        const data = [
          { name: "C", time: 10.77 },
          { name: "Rust", time: 21.81 },
          { name: "Ruchy (C)", time: 22.47 },
          { name: "Ruchy (T)", time: 23.68 },
          { name: "Go", time: 38.04 },
          { name: "Deno", time: 70.11 },
          { name: "Julia 🧪", time: 252.91 },
          { name: "Python", time: 697.49 },
        ];
        return "```\n" + generatePerformanceChart(data, 70) + "\n```";
      },
    },
    {
      name: "Size Chart",
      startMarker: "<!-- AUTO-GENERATED-CHART: size-start -->",
      endMarker: "<!-- AUTO-GENERATED-CHART: size-end -->",
      generator: () => {
        const data = [
          { name: "Ruchy (T)", value: 312, unit: "KB" },
          { name: "Ruchy (C)", value: 314, unit: "KB" },
          { name: "Rust", value: 424, unit: "KB" },
          { name: "C", value: 695, unit: "KB" },
          { name: "Go", value: 1.41, unit: "MB" },
          { name: "Python", value: 119, unit: "MB" },
          { name: "Deno", value: 256, unit: "MB" },
          { name: "Julia 🧪", value: 711, unit: "MB" },
        ];
        return "```\n" + generateSizeChart(data, 70) + "\n```";
      },
    },
  ];

  // Read README
  let content = await Deno.readTextFile(readmePath);

  // Create backup
  await createBackup(readmePath);

  // Update each section
  let allSuccessful = true;
  for (const section of sections) {
    console.log(`📊 Updating ${section.name}...`);

    const newContent = section.generator();
    const result = replaceSectionBetweenMarkers(
      content,
      section.startMarker,
      section.endMarker,
      newContent,
    );

    if (result.success) {
      content = result.content;
      console.log(`   ✅ ${section.name} updated`);
    } else {
      console.error(`   ❌ ${section.name} failed: ${result.error}`);
      allSuccessful = false;
    }
  }

  if (!allSuccessful) {
    console.error("\n❌ Some sections failed to update. README not modified.");
    Deno.exit(1);
  }

  // Write updated content
  await Deno.writeTextFile(readmePath, content);

  console.log("\n✅ README.md successfully updated!");
  console.log("   All charts are now proportional and accurate.");
}

// Main execution
if (import.meta.main) {
  const readmePath = "./README.md";

  try {
    await updateReadmeCharts(readmePath);
  } catch (error) {
    console.error("❌ Error:", error.message);
    Deno.exit(1);
  }
}
