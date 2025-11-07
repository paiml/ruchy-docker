# Scripts Documentation

## Chart Generation & README Updates

### Overview

This directory contains scripts for generating proportional ASCII charts and safely updating the README.md file.

### Files

- **`chart-generator.ts`** - Reusable library for generating proportional ASCII charts
- **`generate-ascii-charts.ts`** - Standalone script to preview charts
- **`update-readme-charts.ts`** - Safely updates chart sections in README.md

### Usage

#### 1. Preview Charts

Generate and preview charts without modifying files:

```bash
deno run scripts/generate-ascii-charts.ts
```

#### 2. Update README.md

Automatically update chart sections in README.md:

```bash
deno run --allow-read --allow-write scripts/update-readme-charts.ts
```

**Features:**
- ✅ Creates timestamped backup (e.g., `README.md.backup-2025-11-07T05-02-10-809Z`)
- ✅ Only replaces content between markers
- ✅ Validates all markers exist before modifying
- ✅ Atomic operation - all sections update or none do
- ✅ Clear success/error messages

### How It Works

#### Marker-Based Replacement

README.md contains HTML comment markers that delimit auto-generated sections:

```markdown
<!-- AUTO-GENERATED-CHART: performance-start -->
[chart content here]
<!-- AUTO-GENERATED-CHART: performance-end -->
```

The update script:
1. Reads README.md
2. Finds content between `*-start` and `*-end` markers
3. Generates new chart using proportional scaling
4. Replaces only the content between markers
5. Preserves all other content

#### Proportional Scaling

Charts use mathematical scaling to ensure visual accuracy:

```typescript
// Performance chart (time in ms)
const scale = maxWidth / maxValue;
const barLength = floor(value * scale);

// Example: C=10.77ms, Python=697.49ms, maxWidth=70
// scale = 70 / 697.49 = 0.1003
// barC = floor(10.77 * 0.1003) = 1 char
// barPython = floor(697.49 * 0.1003) = 70 chars
```

### Adding New Chart Sections

To add a new auto-generated section to README.md:

1. **Add markers to README.md:**
   ```markdown
   <!-- AUTO-GENERATED-CHART: my-chart-start -->
   [initial content]
   <!-- AUTO-GENERATED-CHART: my-chart-end -->
   ```

2. **Add section to `update-readme-charts.ts`:**
   ```typescript
   const sections: ChartSection[] = [
     // ... existing sections ...
     {
       name: "My Chart",
       startMarker: "<!-- AUTO-GENERATED-CHART: my-chart-start -->",
       endMarker: "<!-- AUTO-GENERATED-CHART: my-chart-end -->",
       generator: () => {
         const data = [...]; // your data
         return "```\\n" + generateMyChart(data) + "\\n```";
       },
     },
   ];
   ```

3. **Test:**
   ```bash
   deno run --allow-read --allow-write scripts/update-readme-charts.ts
   ```

### Safety Features

1. **Automatic Backups**
   - Created before any modification
   - Timestamped for easy tracking
   - Located in project root

2. **Validation**
   - Checks all markers exist before modifying
   - Fails fast if any section missing
   - No partial updates

3. **Clear Output**
   ```
   🔧 Updating README.md charts...
   ✅ Backup created: ./README.md.backup-2025-11-07T05-02-10-809Z
   📊 Updating Performance Chart...
      ✅ Performance Chart updated
   📊 Updating Size Chart...
      ✅ Size Chart updated
   ✅ README.md successfully updated!
   ```

### Updating Chart Data

When benchmark results change:

1. **Edit data in `generate-ascii-charts.ts`:**
   ```typescript
   const performanceData: PerformanceData[] = [
     { name: "C", time: 10.77 },
     { name: "Deno", time: 70.11 },  // Update this
     // ...
   ];
   ```

2. **Copy same data to `update-readme-charts.ts`** (or refactor to shared config)

3. **Run update script:**
   ```bash
   deno run --allow-read --allow-write scripts/update-readme-charts.ts
   ```

### Best Practices

✅ **DO:**
- Preview charts with `generate-ascii-charts.ts` before updating README
- Keep backup files for at least one release cycle
- Update both scripts when data changes
- Verify markers exist before running update

❌ **DON'T:**
- Manually edit content between auto-generated markers (it will be overwritten)
- Delete backup files immediately
- Remove or modify marker comments

### Troubleshooting

**Error: "Start marker not found"**
- Check that marker comments exist in README.md
- Verify exact marker text matches (case-sensitive)

**Error: "Invalid marker structure"**
- Ensure `*-start` marker comes before `*-end` marker
- Check no nested markers exist

**Charts look wrong**
- Verify data values are correct
- Check maxWidth parameter (default 70 chars)
- Test with `generate-ascii-charts.ts` first

### Example Workflow

```bash
# 1. Update benchmark data in scripts
vim scripts/generate-ascii-charts.ts

# 2. Preview new charts
deno run scripts/generate-ascii-charts.ts

# 3. If satisfied, update README.md
deno run --allow-read --allow-write scripts/update-readme-charts.ts

# 4. Verify changes
git diff README.md

# 5. Commit
git add README.md scripts/
git commit -m "Update benchmark charts with new Deno results"
```

---

**No More Manual Chart Mistakes!** 🎉

The marker-based system ensures charts stay accurate and proportional without manual editing errors.
