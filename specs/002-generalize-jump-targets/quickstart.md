# Quickstart: Generalize Jump Targets

**Feature Branch**: `002-generalize-jump-targets`
**Date**: 2025-12-13

## Overview

This feature extends the `jump` command to support URLs and arbitrary strings in addition to file paths. Target type is detected automatically by prefix.

## Target Types

| Prefix | Type | Behavior |
|--------|------|----------|
| `http://` or `https://` | URL | Output verbatim |
| `/`, `~`, `$`, `%` | Path | Apply expansion (tilde, env vars, dates) |
| (anything else) | String | Output verbatim |

## Usage Examples

### URLs

```csv
# In ~/.config/jump/targets.csv
https://github.com/user/repo,gh
https://jira.example.com/browse/PROJ-123,ticket
http://localhost:3000,local
```

```bash
$ jump gh
https://github.com/user/repo

$ jump ticket
https://jira.example.com/browse/PROJ-123

# Use with browser
$ open $(jump gh)
```

### Arbitrary Strings

```csv
# Issue/PR identifiers (note: values starting with # work, but lines starting with # are comments)
PR-789,pr
ISSUE-456,issue

# SSH targets
user@prod.example.com:/var/app,ssh-prod

# Custom identifiers
PROJ-2024-001,project-id
```

```bash
$ jump pr
PR-789

$ jump ssh-prod
user@prod.example.com:/var/app

# Use with SSH
$ ssh $(jump ssh-prod)
```

### Paths (unchanged)

```csv
# These work exactly as before
~/projects,prj
$HOME/documents,docs
/var/log/%Y-%m-%d,logs
```

```bash
$ jump prj
/Users/jeff/projects

$ cd $(jump prj)
```

## Configuration

No configuration changes required. The existing `targets.csv` format is fully compatible.

**Location**: `~/.config/jump/targets.csv` (or paths in `$JUMP_PREFIXES`)

**Format**: `value,name1,name2,...`

## Building

```bash
cd ~/conf/prj/jump
cargo build --release
cargo install --path .
```

## Testing

```bash
cd ~/conf/prj/jump
cargo test
```

## Backward Compatibility

- All existing path targets continue to work
- No changes to CSV format
- No changes to CLI interface (`jump <name>`)
