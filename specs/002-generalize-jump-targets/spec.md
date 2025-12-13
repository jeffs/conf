# Feature Specification: Generalize Jump Targets

**Feature Branch**: `002-generalize-jump-targets`
**Created**: 2025-12-13
**Status**: Draft
**Input**: User description: "The jump command currently supports target paths. Generalize it so that targets can be arbitrary strings, such as URLs, or Pull Request or Issue IDs."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - URL Targets (Priority: P1)

As a user, I want to store URLs in my jump targets database so that I can quickly retrieve frequently-accessed web addresses using short memorable names.

**Why this priority**: URLs are the most commonly needed non-path target type. The code already has a placeholder for URL support (`expansion.rs:110-114`), indicating this was always intended.

**Independent Test**: Can be fully tested by adding a URL target to `targets.csv`, running `jump <name>`, and verifying the URL is output exactly as stored.

**Acceptance Scenarios**:

1. **Given** a target entry `https://github.com/user/repo,gh-repo` exists in the database, **When** the user runs `jump gh-repo`, **Then** `https://github.com/user/repo` is output to stdout
2. **Given** a target entry `http://localhost:3000,local` exists, **When** the user runs `jump local`, **Then** `http://localhost:3000` is output to stdout
3. **Given** a URL target with query parameters `https://example.com/search?q=test,search` exists, **When** the user runs `jump search`, **Then** the complete URL including query parameters is output

---

### User Story 2 - Arbitrary String Targets (Priority: P2)

As a user, I want to store arbitrary strings (like PR numbers, issue IDs, or custom identifiers) so that I can use jump as a general-purpose shortcut lookup tool.

**Why this priority**: Extends the utility beyond paths and URLs to support any string-based shortcuts, enabling workflows like `jump pr-123` to output `#123` or similar identifiers.

**Independent Test**: Can be tested by adding non-path, non-URL entries to `targets.csv` and verifying they are output exactly as stored.

**Acceptance Scenarios**:

1. **Given** a target entry `#456,current-issue` exists, **When** the user runs `jump current-issue`, **Then** `#456` is output to stdout
2. **Given** a target entry `PR-789,pr` exists, **When** the user runs `jump pr`, **Then** `PR-789` is output to stdout
3. **Given** a target entry with special characters `user@host:path,ssh-target` exists, **When** the user runs `jump ssh-target`, **Then** `user@host:path` is output exactly

---

### User Story 3 - Backward Compatibility with Paths (Priority: P1)

As an existing user, I want my current path-based targets to continue working with all expansion features (tilde, environment variables, date formatting) so that my workflow is not disrupted.

**Why this priority**: Existing users must not experience breaking changes. Path expansion is core functionality that must be preserved.

**Independent Test**: Can be tested by running existing path targets and verifying tilde expansion, variable substitution, and date formatting all work correctly.

**Acceptance Scenarios**:

1. **Given** a target entry `~/projects,prj` exists, **When** the user runs `jump prj`, **Then** the output is the expanded path (e.g., `/Users/jeff/projects`)
2. **Given** a target entry `$HOME/documents,docs` exists, **When** the user runs `jump docs`, **Then** the environment variable is expanded in the output
3. **Given** a target entry `/var/log/%Y-%m-%d,today-logs` exists, **When** the user runs `jump today-logs`, **Then** the date placeholder is expanded to today's date

---

### Edge Cases

- What happens when a target value looks like a path but is meant to be literal? Users can prefix with a marker if needed, but by default strings starting with `/`, `~`, or `$` will be treated as paths and expanded.
- What happens when a URL contains a tilde or dollar sign in the path portion? URLs are detected by their scheme (`http://` or `https://`) and are output verbatim without expansion.
- What happens when a target value is empty? An appropriate error message is displayed, consistent with current behavior.
- What happens with relative paths vs. arbitrary strings? Strings without path-like prefixes (`/`, `~`, `$`, `%`) and without URL schemes are treated as arbitrary strings and output verbatim.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST detect URL targets by the presence of `http://` or `https://` scheme prefix
- **FR-002**: System MUST output URL targets verbatim without any expansion or modification
- **FR-003**: System MUST continue to expand path targets that begin with `/`, `~`, `$`, or `%`
- **FR-004**: System MUST output arbitrary string targets (not matching path or URL patterns) verbatim
- **FR-005**: System MUST preserve all characters in URL targets, including query strings and fragments
- **FR-006**: System MUST maintain backward compatibility with existing path-based targets
- **FR-007**: System MUST NOT require changes to the `targets.csv` file format

### Target Type Detection Rules

The system determines target type by examining the stored value:

1. **URL**: Value starts with `http://` or `https://` → output verbatim
2. **Path**: Value starts with `/`, `~`, `$`, or `%` → apply path expansion rules
3. **Arbitrary String**: All other values → output verbatim

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of URL targets are output exactly as stored in the database
- **SC-002**: 100% of existing path-based targets continue to work with expansion
- **SC-003**: Users can retrieve any stored string value using jump in a single command
- **SC-004**: No changes required to existing `targets.csv` files for path targets

## Assumptions

- Users understand that values starting with `/`, `~`, `$`, or `%` will be treated as paths with expansion
- The distinction between target types is based on the stored value's prefix/scheme, not metadata
- URL validation (whether the URL is well-formed) is not required; the tool outputs what is stored
