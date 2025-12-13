# Tasks: Remove Command Mode from Jump

**Input**: Design documents from `/specs/001-remove-jump-command-mode/`
**Prerequisites**: plan.md, spec.md, research.md

**Tests**: No tests requested in feature specification. Manual verification per quickstart.md.

**Organization**: Tasks grouped by user story for independent implementation.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2)
- Include exact file paths in descriptions

## Path Conventions

- **Project root**: `prj/jump/`
- **Source**: `prj/jump/src/`

---

## Phase 1: Setup

**Purpose**: Verify build environment and understand current state

- [x] T001 Verify project builds successfully with `cargo build` in prj/jump/
- [x] T002 Review current CLI behavior by testing `jump --help` or similar

**Checkpoint**: Build working, baseline established

---

## Phase 2: Foundational (Core Removal)

**Purpose**: Remove command infrastructure from expansion module - this MUST complete before CLI changes

**⚠️ CRITICAL**: The CLI depends on expansion.rs changes being complete first

- [x] T003 Remove `cmd` module (constants `CD`, `OPEN`) from prj/jump/src/expansion.rs lines 15-24
- [x] T004 Remove `command!` macro from prj/jump/src/expansion.rs lines 63-74
- [x] T005 Remove `Expand::command()` method from prj/jump/src/expansion.rs lines 149-166
- [x] T006 Remove `App::command()` method from prj/jump/src/lib.rs lines 95-103
- [x] T007 Verify build succeeds after expansion/lib removals with `cargo build` in prj/jump/

**Checkpoint**: Command generation infrastructure removed, build passing

---

## Phase 3: User Story 1 - Simplified Jump Interface (Priority: P1) 🎯 MVP

**Goal**: Remove command mode from CLI so jump only returns paths

**Independent Test**: Run `jump <target>` and verify it returns a path; run `jump -c <target>` and verify error

### Implementation for User Story 1

- [x] T008 [US1] Remove `is_command` field from `Args` struct in prj/jump/src/main.rs line 68
- [x] T009 [US1] Remove `-c`/`--command` flag parsing from `parse_args()` in prj/jump/src/main.rs lines 76-77
- [x] T010 [US1] Remove command mode branch from `main_imp()` in prj/jump/src/main.rs lines 98-99
- [x] T011 [US1] Simplify `main_imp()` to only call `app.path()` in prj/jump/src/main.rs
- [x] T012 [US1] Verify build succeeds with `cargo build` in prj/jump/

**Checkpoint**: User Story 1 complete - jump only returns paths, `-c` flag rejected

---

## Phase 4: User Story 2 - Clean Error Messages (Priority: P2)

**Goal**: Ensure clear error messages when deprecated flags are used

**Independent Test**: Run `jump -c target` and `jump --command target`, verify error messages are clear

### Implementation for User Story 2

- [x] T013 [US2] Verify `-c` flag produces "not a recognized flag" error by testing `jump -c test`
- [x] T014 [US2] Verify `--command` flag produces "not a recognized flag" error by testing `jump --command test`
- [x] T015 [US2] Verify non-zero exit code is returned for unrecognized flags

**Checkpoint**: User Story 2 complete - deprecated flags produce clear errors

---

## Phase 5: Polish & Cleanup

**Purpose**: Remove any orphaned code, verify final state

- [x] T016 [P] Check if `as_bytes.rs` has any remaining usages with `cargo build` warnings in prj/jump/
- [x] T017 [P] Remove `as_bytes.rs` if unused (or document why retained) in prj/jump/src/
- [x] T018 [P] Remove `use std::io::Write` import if no longer needed in prj/jump/src/expansion.rs
- [x] T019 Run `cargo clippy` to identify any remaining dead code in prj/jump/
- [x] T020 Verify all acceptance scenarios from spec.md pass with manual testing
- [x] T021 Update prj/jump/TODO.md to remove command-related items if present

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup - BLOCKS User Story 1
- **User Story 1 (Phase 3)**: Depends on Foundational completion
- **User Story 2 (Phase 4)**: Depends on User Story 1 (error behavior inherent from removal)
- **Polish (Phase 5)**: Depends on User Stories 1 and 2

### User Story Dependencies

- **User Story 1 (P1)**: Must complete foundational removal first (T003-T007)
- **User Story 2 (P2)**: Error messages are automatic from removal; just needs verification

### Within Phases

- Foundational tasks T003-T006 can run in sequence (same/related files)
- T007 must follow T003-T006 (build verification)
- User Story 1 tasks T008-T011 are sequential (same file, dependent changes)
- T012 must follow T008-T011 (build verification)
- Polish tasks T016-T018 marked [P] can run in parallel

### Parallel Opportunities

- T016, T017, T018 can run in parallel (different concerns)
- User Story 2 verification tasks T013-T015 can run in parallel

---

## Parallel Example: Foundational Phase

```bash
# Sequential within Foundational (same file modifications):
T003 → T004 → T005 → T006 → T007

# But Polish tasks can be parallelized:
Task: "Check if as_bytes.rs has remaining usages"
Task: "Remove use std::io::Write import if no longer needed"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T002)
2. Complete Phase 2: Foundational (T003-T007)
3. Complete Phase 3: User Story 1 (T008-T012)
4. **STOP and VALIDATE**: Test `jump <target>` works, `jump -c <target>` fails
5. MVP complete - core functionality delivered

### Full Delivery

1. Complete MVP (Phases 1-3)
2. Add User Story 2 verification (Phase 4: T013-T015)
3. Polish and cleanup (Phase 5: T016-T021)
4. All acceptance criteria met

---

## Notes

- This is a removal feature - tasks primarily delete code
- Rust compiler will catch missing removals (unused code warnings, build errors)
- No new tests needed - verification is manual per quickstart.md
- Total removal: ~40 lines of code across 3 files
- Commit after each phase for clean history
