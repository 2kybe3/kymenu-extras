**kymenu-dir** - Recursively lists directories and files with filtering and formatting options for kyvim integration.

### Features

- Choose display mode: `relative` (default), `relative-prefixed`, `filename`, or `absolute`
- Support for `--min-depth` and `--max-depth`
- Filter with `--file` or `--folder`

### Usage

```bash
# Basic usage (relative paths)
kymenu-dir ~/Downloads | kymenu --json-in

# Multiple Paths
kymenu-dir --mode relative-prefixed ~/git ~/Projects | kymenu --json-in

# Show only filenames
kymenu-dir . --mode filename | kymenu --json-in

# Full absolute paths
kymenu-dir /etc --mode absolute | kymenu --json-in

# Limit depth
kymenu-dir ~/git --max-depth 1 | kymenu --json-in
```
