# Documentation Images

Visual assets for the ruchy-docker project documentation.

## Files

### runtime-problems.png (193 KB)
**Size**: 1400×900 pixels
**Format**: PNG
**Purpose**: Comprehensive scientific analysis of Docker interpreted runtime problems

Used in [docs/runtime.md](../runtime.md) to illustrate the three main problems with interpreted languages in Docker:

1. **Runtime Size Bloat**: 280-2,284× larger binaries (87-711 MB vs 0.31 MB)
2. **Execution Time Penalty**: 6.5-64.8× slower (70-697 ms vs 10.8 ms)
3. **Technical Debt**: Dependency hell, security attack surface, cold start latency, memory overhead

**Visualization Components**:
- Side-by-side bar charts (size + performance)
- 4 technical issue cards with details
- Color-coded by language type (compiled green, bundled orange, interpreted red)
- Solution summary highlighting Ruchy's advantages

### runtime-problems.svg (9.0 KB)
**Purpose**: Source SVG for runtime-problems.png (editable vector format)

**Regenerating PNG**:
```bash
convert runtime-problems.svg -resize 1400x900 -background none runtime-problems.png
```

---

### runtime-comparison.png (176 KB)
**Size**: 1400×900 pixels
**Format**: PNG
**Purpose**: Visual comparison of Python interpreter vs compiled binary runtime sizes

Used in main README.md to illustrate the fundamental difference between:
- **Interpreted languages** (Python, Julia): Ship entire runtime + interpreter + libraries (100+ MB)
- **Compiled languages** (C, Rust, Go, Ruchy): Ship only machine code (<1 MB)

**Key Points Illustrated**:
- Python 3.12-slim: 119 MB (interpreter + stdlib + minimal OS)
- NumPy package: 31 MB (C extensions + Python code)
- NumPy libraries: 27 MB (OpenBLAS + Fortran runtime)
- **Total Python + NumPy**: 177 MB

vs.

- Compiled binaries: <1 MB (just your code + minimal stdlib)
- **177× size difference**
- **64× performance difference**

### runtime-comparison.svg (5.9 KB)
**Purpose**: Source SVG for the PNG image (editable vector format)

**Editing**: Use any SVG editor (Inkscape, Figma, etc.) or edit XML directly

**Regenerating PNG**:
```bash
convert runtime-comparison.svg -resize 1400x900 -background none runtime-comparison.png
```

## Usage in README

```markdown
![Runtime Size Comparison](docs/images/runtime-comparison.png)
```

## Design Rationale

The visualization uses:
- **Proportional sizing**: Python side (530px height) vs Compiled side (50px square) reflects actual 177× ratio
- **Color coding**:
  - Red (#ef4444): Python runtime overhead
  - Orange (#f59e0b): NumPy package
  - Dark red (#dc2626): NumPy libraries
  - Green (#10b981): Compiled solution
- **Clear breakdown**: Stacked rectangles show where the 177 MB comes from
- **Educational labels**: Explains what each component does and why it's needed

## License

MIT (same as project)
