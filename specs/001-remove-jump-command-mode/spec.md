# Feature Specification: Remove Command Mode from Jump

**Feature Branch**: `001-remove-jump-command-mode`
**Created**: 2025-12-12
**Status**: Draft
**Input**: User description: "The `jump` command (`prj/jump/src/main.rs`) can retrieve either a path or a shell command. Remove the 'command' mode support."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Simplified Jump Interface (Priority: P1)

As a user of the jump command, I want the tool to only return directory paths so that the interface is simpler and more predictable.

**Why this priority**: This is the core change - removing the command mode means users will always get a path, making the tool's behavior consistent and easier to understand.

**Independent Test**: Can be fully tested by running `jump <target>` and verifying it always returns a path, never a shell command.

**Acceptance Scenarios**:

1. **Given** the jump command is invoked with a valid target, **When** the target maps to a directory path, **Then** the path is returned to stdout
2. **Given** the jump command is invoked with `-c` or `--command` flag, **When** the command is executed, **Then** an error message indicates the flag is not recognized
3. **Given** the jump command is invoked with a valid target, **When** the user attempts to use command mode, **Then** only path mode behavior occurs (no command execution)

---

### User Story 2 - Clean Error Messages (Priority: P2)

As a user who previously used command mode, I want clear feedback when using deprecated flags so that I understand the tool's current capabilities.

**Why this priority**: Users migrating from the old interface need clear guidance that command mode no longer exists.

**Independent Test**: Can be tested by invoking `jump -c <target>` or `jump --command <target>` and verifying an appropriate error message is displayed.

**Acceptance Scenarios**:

1. **Given** a user invokes jump with the `-c` flag, **When** the command runs, **Then** an error message explains that `-c` is not a recognized flag
2. **Given** a user invokes jump with the `--command` flag, **When** the command runs, **Then** an error message explains that `--command` is not a recognized flag

---

### Edge Cases

- What happens when a target in the database previously contained command data? The path method should still work for path targets; command-specific targets will no longer be accessible through the CLI.
- How does the system handle existing shell aliases or scripts that use the `-c` flag? They will receive an error message and a non-zero exit code.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST NOT accept the `-c` flag as a valid command-line argument
- **FR-002**: System MUST NOT accept the `--command` flag as a valid command-line argument
- **FR-003**: System MUST return an "unrecognized flag" error when `-c` or `--command` is used
- **FR-004**: System MUST continue to support path lookup for valid targets
- **FR-005**: System MUST return a non-zero exit code when invalid flags are provided
- **FR-006**: System MUST NOT execute shell commands under any circumstances
- **FR-007**: The library MUST NOT provide any command-generation functionality to consumers

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of invocations with `-c` or `--command` flags result in an error message
- **SC-002**: All existing path-based targets continue to resolve successfully
- **SC-003**: The command-line interface has one fewer mode of operation (reduced from 2 to 1)
- **SC-004**: Users receive immediate feedback (error message) when attempting to use removed functionality

## Assumptions

- Existing path targets in `targets.csv` will continue to work without modification
- Users who relied on command mode will need to find alternative solutions for their workflow
- The database format does not need to change

## Scope

This change removes command mode support **entirely** from the codebase:

- **CLI** (`main.rs`): Remove `-c`/`--command` flag handling and `is_command` field
- **Library** (`lib.rs`): Remove `App::command()` method
- **Expansion** (`expansion.rs`): Remove `Expand::command()` method and related shell command infrastructure

After this change, no consumers (internal or external) will have access to command mode functionality.
