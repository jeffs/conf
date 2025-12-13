# Research: Remove Command Mode from Jump

**Date**: 2025-12-12
**Feature**: 001-remove-jump-command-mode

## Overview

This document captures research findings for removing command mode support from the `jump` CLI tool. Since this is a code removal feature with no external dependencies or unknowns, the research focuses on identifying all code paths to remove.

## Code Analysis

### Components to Remove

#### 1. CLI Layer (`main.rs`)

| Item | Lines | Description |
|------|-------|-------------|
| `Args.is_command` field | 68 | Boolean tracking command mode flag |
| `-c`/`--command` parsing | 76-77 | Flag detection in `parse_args()` |
| Command mode branch | 98-99 | `if args.is_command { ... }` in `main_imp()` |

**Decision**: Remove `is_command` field and all related branching logic. Simplify `Args` struct to only contain `target`.

**Rationale**: With command mode removed, the struct and parsing become simpler. No alternative needed.

#### 2. Library API (`lib.rs`)

| Item | Lines | Description |
|------|-------|-------------|
| `App::command()` method | 100-103 | Public method returning shell command bytes |

**Decision**: Remove `App::command()` method entirely.

**Rationale**: FR-007 requires library must not provide command-generation functionality. No consumers exist.

**Alternatives Considered**: Deprecation annotation - rejected because no external consumers and spec requires full removal.

#### 3. Expansion Module (`expansion.rs`)

| Item | Lines | Description |
|------|-------|-------------|
| `cmd` module | 15-24 | Constants for shell commands (`CD`, `OPEN`) |
| `command!` macro | 63-74 | Macro generating shell command output |
| `Expand::command()` method | 157-166 | Method wrapping path in shell command |

**Decision**: Remove `cmd` module, `command!` macro, and `Expand::command()` method.

**Rationale**: All shell command generation infrastructure is no longer needed.

**Alternatives Considered**: Keep `cmd` module for future use - rejected per Minimal Dependencies principle (don't keep unused code).

### Components to Retain

#### `as_bytes.rs`

The `AsBytes` trait and implementations are used by:
- `command!` macro (being removed)
- Potentially `main.rs` output writing

**Analysis**: The macro `command!` uses `$arg.as_bytes()` on Path-like types. After removal, check if `AsBytes` is still needed elsewhere. If only used by command code, remove it too.

**Decision**: Review after primary removal. May be removable if unused.

#### `error.rs`

All error variants (`Database`, `Expansion`, `Target`) are used by path functionality and should be retained.

**Decision**: No changes needed.

## Dependency Analysis

### `chrono` Dependency

Used for date expansion in paths (e.g., `%Y-%m-%d` patterns). Not related to command mode.

**Decision**: Retain. Still needed for path expansion.

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Breaking existing scripts using `-c` | Low | Medium | Error message clearly indicates flag not recognized |
| Orphaned dead code | Low | Low | Compiler warnings will flag unused items |
| Missing removal items | Low | Low | `cargo build` will fail on references to removed items |

## Summary

This is a straightforward removal with clear boundaries:

1. **Remove** (required):
   - `main.rs`: `is_command` field and flag handling
   - `lib.rs`: `App::command()` method
   - `expansion.rs`: `cmd` module, `command!` macro, `Expand::command()` method

2. **Review** (conditional):
   - `as_bytes.rs`: May be removable if only used by command code

3. **Retain** (no changes):
   - `error.rs`: All variants still used
   - `db.rs`: Unrelated to command mode
   - `chrono` dependency: Still needed for date expansion
