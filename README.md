**kymenu-extras** — Companion tools for kymenu

Lightweight utilities that generate input for **kymenu**.

### `kymenu-dir`

Recursively walks a directory and outputs items suitable for piping into `kymenu`.

### Features
- Choose display mode: `relative` (default), `filename`, or `absolute`
- Support for `--min-depth` and `--max-depth`
- Filter with `--file` or `--folder`

### Usage

```bash
# Basic usage (relative paths)
kymenu-dir ~/Downloads | kymenu --json-in

# Show only filenames
kymenu-dir . --mode filename | kymenu --json-in

# Full absolute paths
kymenu-dir /etc --mode absolute | kymenu --json-in

# Limit depth
kymenu-dir ~/git --max-depth 1 | kymenu --json-in
```

### Options

```bash
kymenu-dir <PATH> [--mode <MODE>] [--max-depth N] [--min-depth N] [--file true/false] [--foler true/false]
```

**Modes**: `relative` (default), `filename`, `absolute`
