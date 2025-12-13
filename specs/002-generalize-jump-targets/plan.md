# Implementation Plan: Generalize Jump Targets

**Branch**: `002-generalize-jump-targets` | **Date**: 2025-12-13 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-generalize-jump-targets/spec.md`

## Summary

Generalize the `jump` command to support arbitrary string targets (URLs, PR/issue IDs) in addition to file paths. The system will detect target type by prefix: URLs (`http://`, `https://`) and paths (`/`, `~`, `$`, `%`) receive special handling, while all other strings are output verbatim. This extends the existing `expansion.rs` module which already has placeholder code for URL support.

## Technical Context

**Language/Version**: Rust 2024 edition (nightly features)
**Primary Dependencies**: chrono 0.4.40 (retained for date expansion in paths)
**Storage**: CSV files (`~/.config/jump/targets.csv` or `$JUMP_PREFIXES`)
**Testing**: cargo test
**Target Platform**: macOS (primary), Unix (portable)
**Project Type**: Single CLI application
**Performance Goals**: Instant lookup (<10ms for typical use)
**Constraints**: No new dependencies; backward compatibility required
**Scale/Scope**: Single-user CLI tool, ~100-1000 targets typical

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Minimal Dependencies | ✅ PASS | No new dependencies required; chrono already present |
| II. Simple Installation | ✅ PASS | No installation changes; existing targets.csv format preserved |
| III. Consistency | ✅ PASS | Follows existing patterns in expansion.rs; target type detection mirrors existing special character handling |
| IV. Code Quality | ✅ PASS | Rust implementation; explicit error handling; extends existing module structure |
| V. macOS-First Portability | ✅ PASS | No platform-specific code changes; Unix-compatible |

**Gate Result**: PASS - All principles satisfied. No violations to justify.

## Project Structure

### Documentation (this feature)

```text
specs/002-generalize-jump-targets/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (N/A for CLI)
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
prj/jump/
├── Cargo.toml
└── src/
    ├── main.rs          # CLI entry point (minor changes: output type)
    ├── lib.rs           # App struct (change return type from PathBuf to Target)
    ├── db.rs            # Database (change storage from PathBuf to String)
    ├── expansion.rs     # Expand struct (implement target type detection and routing)
    └── error.rs         # Error types (no changes expected)
```

**Structure Decision**: Single project structure maintained. Changes are localized to the existing `prj/jump/` directory. The expansion module is the primary change location, with supporting changes to db.rs (storage type) and lib.rs (return type).

## Complexity Tracking

> No violations to justify - all constitution principles pass.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | N/A | N/A |
