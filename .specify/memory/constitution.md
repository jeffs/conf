<!--
Sync Impact Report
==================
Version change: 0.0.0 → 1.0.0
MAJOR bump rationale: Initial constitution ratification (new governance document)

Modified principles: N/A (initial version)
Added sections:
  - Core Principles (5 principles)
  - Installation & Upgrade Standards
  - Development Workflow
  - Governance

Removed sections: N/A (initial version)

Templates requiring updates:
  - .specify/templates/plan-template.md: ✅ No changes needed (Constitution Check is generic)
  - .specify/templates/spec-template.md: ✅ No changes needed (requirements section compatible)
  - .specify/templates/tasks-template.md: ✅ No changes needed (task structure compatible)

Follow-up TODOs: None
-->

# Conf Constitution

## Core Principles

### I. Minimal Dependencies

All configuration and tooling MUST minimize external dependencies:

- Prefer system-provided tools over third-party alternatives when functionality is equivalent
- Each external dependency MUST be justified with a clear rationale documenting why built-in
  alternatives are insufficient
- Homebrew is the primary package manager; additional package managers (cargo, npm, go) are
  permitted only for language-specific tooling not available via Homebrew
- Dependencies MUST be auditable: no opaque binary blobs or unvetted repositories

**Rationale**: Fewer dependencies means fewer failure points, faster setup, reduced security
surface, and easier maintenance across machine migrations.

### II. Simple Installation

Installation and upgrade processes MUST be simple, safe, and repeatable:

- All install scripts MUST be idempotent: running them multiple times produces the same result
  without side effects
- Scripts MUST support both fresh installation and incremental upgrades
- Version changes MUST be logged (before/after) during upgrades for auditability
- Uninstallation/reversal MUST be possible for any installed component
- Scripts MUST fail fast with clear error messages rather than partially completing

**Rationale**: Configuration management is maintenance-heavy; simple installation reduces cognitive
load and enables confident experimentation.

### III. Consistency

The codebase MUST maintain consistency in style, structure, and behavior:

- Similar operations MUST use similar patterns across different tools and configurations
- Naming conventions MUST be uniform (files, functions, environment variables)
- Directory structure MUST follow established conventions documented in the repository
- Configuration files MUST use consistent formatting within each tool's ecosystem
- Behavior MUST be predictable: same inputs produce same outputs across invocations

**Rationale**: Consistency reduces surprises, makes the codebase navigable, and lowers the barrier
to making changes confidently.

### IV. Code Quality

Code MUST prioritize clarity and reliability:

- Rust is the preferred language for new tooling where feasible; shell scripts are acceptable for
  simple glue code and installation procedures
- All code MUST be readable without extensive comments; comments explain *why*, not *what*
- Error handling MUST be explicit: no silent failures, swallowed exceptions, or ambiguous states
- Complex logic MUST be factored into named functions with clear purposes
- Shell scripts MUST use strict mode (`set -euo pipefail` or equivalent) where supported

**Rationale**: Well-written code is easier to debug, extend, and trust. Rust provides safety
guarantees that shell scripts cannot.

### V. macOS-First Portability

Configuration MUST be portable across Unix systems without degrading the macOS experience:

- macOS is the primary platform; all features MUST work excellently on macOS
- Linux compatibility is desirable but MUST NOT compromise macOS functionality or user experience
- Platform-specific code MUST be clearly isolated and documented
- When platform behavior differs, macOS behavior is the reference implementation
- Features requiring platform-specific implementations MUST degrade gracefully on unsupported
  platforms rather than failing entirely

**Rationale**: This is a personal configuration repository where macOS is the daily driver;
portability is valuable for servers and containers but not at the cost of primary platform quality.

## Installation & Upgrade Standards

Installation procedures MUST adhere to the following:

- Entry point: `bin/init.zsh` (or equivalent) for full system setup
- Modular installers: `bin/install-*.zsh` for individual components
- Each installer MUST:
  - Check preconditions before making changes
  - Report what will be installed/changed before proceeding
  - Log version information before and after upgrades
  - Exit with non-zero status on failure
- Package installation order MUST respect dependencies
- Environment variables and PATH modifications MUST be documented

## Development Workflow

Contributors (including AI agents) MUST follow this workflow:

- Read existing code before proposing changes
- Verify changes work on macOS before considering cross-platform compatibility
- Test installation scripts in isolation before full-system runs
- Commit atomic changes with clear messages describing *why* the change was made
- Configuration changes MUST be tested by sourcing/reloading the affected shell

## Governance

This constitution governs all development in the `conf` repository:

- **Supremacy**: Constitution principles supersede convenience; violations require explicit
  justification and documentation
- **Amendment process**: Changes to this constitution require:
  1. Clear rationale for the change
  2. Impact assessment on existing code
  3. Version bump following semantic versioning (MAJOR for principle changes, MINOR for additions,
     PATCH for clarifications)
- **Compliance**: All code reviews and AI-assisted development MUST verify alignment with these
  principles
- **Versioning**: MAJOR.MINOR.PATCH format where MAJOR indicates breaking governance changes

**Version**: 1.0.0 | **Ratified**: 2025-12-12 | **Last Amended**: 2025-12-12
