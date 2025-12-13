# Quickstart: Jump (Post Command Mode Removal)

## Overview

The `jump` command resolves target names to directory paths using a CSV-based database.

## Usage

```bash
jump <target>
```

Returns the expanded path for the given target to stdout.

## Examples

```bash
# Look up a target
jump conf
# Output: /Users/jeff/conf

# With date expansion in path
jump logs
# Output: /var/log/2025-12-12

# Tab-completed directory (trailing slash handled)
jump conf/
# Output: /Users/jeff/conf
```

## Error Cases

```bash
# Unknown target
jump nonexistent
# Error: nonexistent: Target not found

# Unrecognized flag (command mode removed)
jump -c conf
# Error: -c is not a recognized flag

jump --command conf
# Error: --command is not a recognized flag
```

## Configuration

Targets are defined in `~/.config/jump/targets.csv`:

```csv
# Format: path,alias1,alias2,...
/Users/jeff/conf,conf,c
/var/log/%Y-%m-%d,logs,l
```

Override search paths with `JUMP_PREFIXES` environment variable (colon-separated).

## Shell Integration

Typical shell function for directory jumping:

```bash
j() {
  local target
  target=$(jump "$1") && cd "$target"
}
```

## What Changed

**Removed in this version:**
- `-c` / `--command` flag: No longer generates shell commands
- Command mode: Tool now exclusively returns paths

If you previously used command mode, update your workflow to use the path output directly with your shell.
