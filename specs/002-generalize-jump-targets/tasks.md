# Tasks: Generalize Jump Targets

**Input**: Design documents from `/specs/002-generalize-jump-targets/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md

**Tests**: No tests explicitly requested in spec. Tasks focus on implementation only.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

Project structure from plan.md:
```text
prj/jump/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    ├── db.rs
    ├── expansion.rs
    └── error.rs
```

---

## Phase 1: Setup

**Purpose**: No setup required - existing project structure is maintained

- [x] T001 Verify project compiles with `cargo build` in prj/jump/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core type changes that MUST be complete before user story features can be implemented

**⚠️ CRITICAL**: All stories depend on the `Target` enum and database storage change

- [x] T002 Define `Target` enum with `Path(PathBuf)` and `String(String)` variants in prj/jump/src/lib.rs
- [x] T003 Change database storage from `HashMap<String, PathBuf>` to `HashMap<String, String>` in prj/jump/src/db.rs
- [x] T004 Update `Database::get()` return type from `Option<&PathBuf>` to `Option<&String>` in prj/jump/src/db.rs
- [x] T005 Verify project still compiles after type changes with `cargo build` in prj/jump/

**Checkpoint**: Foundation ready - type system updated, user story implementation can begin

---

## Phase 3: User Story 3 - Backward Compatibility with Paths (Priority: P1) 🎯 MVP

> **Note**: US3 is implemented before US1 (both P1) to ensure no regression in existing functionality before adding new features. This is a deliberate safety-first ordering.

**Goal**: Preserve existing path expansion functionality (`~`, `$VAR`, `%date`, absolute paths)

**Independent Test**: Run `jump prj` with an existing `~/projects,prj` target and verify tilde expansion works

**Rationale**: Implementing path handling first ensures no regression before adding new features

### Implementation for User Story 3

- [x] T006 [US3] Implement type detection helper function `is_path_target(s: &str) -> bool` that checks for `/`, `~`, `$`, `%` prefixes in prj/jump/src/expansion.rs
- [x] T007 [US3] Rename `Expand::path()` to `Expand::expand_path()` (internal method) in prj/jump/src/expansion.rs
- [x] T008 [US3] Implement `Expand::resolve(&self, value: &str) -> Result<Target>` that routes path values through `expand_path()` and returns `Target::Path` in prj/jump/src/expansion.rs
- [x] T009 [US3] Update `App::path()` to `App::resolve()` returning `Result<Target>` in prj/jump/src/lib.rs
- [x] T010 [US3] Update `main_imp()` to handle `Target::Path` output using `as_os_str().as_bytes()` in prj/jump/src/main.rs
- [x] T011 [US3] Manually test backward compatibility: verify `~/path`, `$HOME/path`, `/absolute/path`, `%Y-%m-%d` targets all expand correctly

**Checkpoint**: Existing path targets work exactly as before - backward compatibility verified

---

## Phase 4: User Story 1 - URL Targets (Priority: P1)

**Goal**: Support URLs (`http://`, `https://`) that are output verbatim without expansion

**Independent Test**: Add `https://github.com/user/repo,gh` to targets.csv and verify `jump gh` outputs the URL exactly

### Implementation for User Story 1

- [x] T012 [US1] Implement type detection helper function `is_url_target(s: &str) -> bool` that checks for `http://` or `https://` prefix in prj/jump/src/expansion.rs
- [x] T013 [US1] Update `Expand::resolve()` to check `is_url_target()` FIRST and return `Target::String(value.to_owned())` for URLs in prj/jump/src/expansion.rs
- [x] T014 [US1] Update `main_imp()` to handle `Target::String` output using `as_bytes()` in prj/jump/src/main.rs
- [x] T015 [US1] Manually test URL targets: verify `http://localhost:3000`, `https://github.com/user/repo`, and `https://example.com/search?q=test&page=1#section` (with query params and fragment) all output verbatim

**Checkpoint**: URL targets work - output exactly as stored without modification

---

## Phase 5: User Story 2 - Arbitrary String Targets (Priority: P2)

**Goal**: Support arbitrary strings (PR IDs, issue numbers, SSH targets) that don't match URL or path patterns

**Independent Test**: Add `#456,issue` to targets.csv and verify `jump issue` outputs `#456` exactly

### Implementation for User Story 2

- [x] T016 [US2] Update `Expand::resolve()` fallback case to return `Target::String(value.to_owned())` for non-URL, non-path values in prj/jump/src/expansion.rs
- [x] T017 [US2] Manually test arbitrary string targets: verify `#456`, `PR-789`, `user@host:path` all output verbatim

**Checkpoint**: All target types (paths, URLs, arbitrary strings) now work correctly

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Cleanup and validation

- [x] T018 [P] Remove `todo!()` placeholders from prj/jump/src/expansion.rs
- [x] T019 [P] Update doc comment for `Expand::path()` rename and new `resolve()` method in prj/jump/src/expansion.rs
- [x] T020 [P] Update doc comment for `App::resolve()` in prj/jump/src/lib.rs
- [x] T021 [P] Remove the `//! TODO: Support URLs, not only paths.` comment from prj/jump/src/expansion.rs (now implemented)
- [x] T022 Run `cargo clippy` and fix any warnings in prj/jump/
- [x] T023 Run `cargo build --release` to verify release build in prj/jump/
- [x] T024 Validate quickstart.md examples work as documented

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - verify existing build
- **Foundational (Phase 2)**: Depends on Setup - BLOCKS all user stories
- **User Story 3 (Phase 3)**: Depends on Foundational - path handling (backward compat)
- **User Story 1 (Phase 4)**: Depends on Foundational; can run parallel to US3 but sequential is safer
- **User Story 2 (Phase 5)**: Depends on Foundational; depends on US3/US1 detection logic being in place
- **Polish (Phase 6)**: Depends on all user stories complete

### User Story Dependencies

- **User Story 3 (P1)**: First priority - ensures no regression
- **User Story 1 (P1)**: Can technically parallel with US3 but shares `expansion.rs` - sequential recommended
- **User Story 2 (P2)**: Relies on detection order from US1/US3 being established

### Within Each User Story

- Detection helper before resolve implementation
- Resolve implementation before main.rs output handling
- Implementation before manual testing

### Parallel Opportunities

- All Phase 6 (Polish) tasks marked [P] can run in parallel
- T002, T003, T004 in Foundational touch different aspects but are sequential for safety

---

## Parallel Example: Phase 6 (Polish)

```bash
# Launch all polish tasks together:
Task: "Remove todo!() placeholders from prj/jump/src/expansion.rs"
Task: "Update doc comment for Expand::path() rename in prj/jump/src/expansion.rs"
Task: "Update doc comment for App::resolve() in prj/jump/src/lib.rs"
Task: "Remove TODO comment from prj/jump/src/expansion.rs"
```

---

## Implementation Strategy

### MVP First (User Story 3 Only)

1. Complete Phase 1: Setup (verify build)
2. Complete Phase 2: Foundational (type changes)
3. Complete Phase 3: User Story 3 (backward compatibility)
4. **STOP and VALIDATE**: Test existing path targets still work
5. This is the minimum viable change - preserves all existing functionality

### Incremental Delivery

1. Complete Setup + Foundational → Type system ready
2. Add User Story 3 → Test path expansion → **MVP: No regression!**
3. Add User Story 1 → Test URLs → **URL support added**
4. Add User Story 2 → Test arbitrary strings → **Full feature complete**
5. Polish phase → Clean code, documentation

### Recommended Execution Order

Given the shared file (`expansion.rs`), sequential execution is recommended:

1. T001 → T005 (Setup + Foundational)
2. T006 → T011 (User Story 3: Paths)
3. T012 → T015 (User Story 1: URLs)
4. T016 → T017 (User Story 2: Arbitrary)
5. T018 → T024 (Polish - can parallelize)

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- This feature has high coupling in `expansion.rs` - most tasks are sequential
- Manual testing is specified since no automated tests were requested
- Commit after each phase checkpoint for clean git history
- The `Target` enum is the key abstraction enabling all three user stories
