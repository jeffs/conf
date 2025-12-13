# Implementation Plan: Remove Command Mode from Jump

**Branch**: `001-remove-jump-command-mode` | **Date**: 2025-12-12 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-remove-jump-command-mode/spec.md`

## Summary

Remove the shell command generation functionality from the `jump` CLI tool entirely. The tool currently supports two modes: returning directory paths (default) and generating shell commands (via `-c`/`--command` flag). This change eliminates command mode from the CLI, library API, and underlying expansion module, simplifying the tool to a single-purpose path resolver.

## Technical Context

**Language/Version**: Rust 2024 edition (nightly features)
**Primary Dependencies**: chrono 0.4.40 (retained for date expansion in paths)
**Storage**: CSV-based target database (`~/.config/jump/targets.csv`)
**Testing**: cargo test (no existing tests observed; manual verification)
**Target Platform**: macOS (primary), Unix-compatible
**Project Type**: Single CLI tool with library
**Performance Goals**: N/A (simple CLI tool)
**Constraints**: N/A (code removal simplifies codebase)
**Scale/Scope**: ~170 lines across 6 source files; removing ~40 lines

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Minimal Dependencies | ✅ PASS | No new dependencies; removes unused code |
| II. Simple Installation | ✅ PASS | No installation changes required |
| III. Consistency | ✅ PASS | Simplifies interface to single behavior pattern |
| IV. Code Quality | ✅ PASS | Removes unused code; Rust codebase maintained |
| V. macOS-First Portability | ✅ PASS | No platform-specific changes |

**Gate Status**: PASS - All principles satisfied. Proceed to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/001-remove-jump-command-mode/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output (N/A for this feature)
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (N/A for this feature)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
prj/jump/
├── Cargo.toml           # Package manifest (no changes needed)
├── install.zsh          # Installation script (no changes needed)
├── TODO.md              # Project notes
└── src/
    ├── main.rs          # CLI entry point - MODIFY: remove -c/--command handling
    ├── lib.rs           # Library API - MODIFY: remove App::command() method
    ├── expansion.rs     # Path/command expansion - MODIFY: remove command() method & cmd module
    ├── error.rs         # Error types (review for unused variants)
    ├── db.rs            # Database handling (no changes)
    └── as_bytes.rs      # Utility trait (review for unused items)
```

**Structure Decision**: Existing single-project CLI structure. Modifications confined to 3 source files (`main.rs`, `lib.rs`, `expansion.rs`) with potential cleanup in `error.rs` and `as_bytes.rs`.

## Complexity Tracking

No violations to justify. This change reduces complexity by removing unused functionality.
