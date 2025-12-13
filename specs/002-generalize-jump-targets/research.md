# Research: Generalize Jump Targets

**Feature Branch**: `002-generalize-jump-targets`
**Date**: 2025-12-13

## Research Tasks

### 1. Target Type Detection Strategy

**Question**: What is the best approach for detecting target types (URL vs Path vs Arbitrary String)?

**Decision**: Prefix-based detection with explicit priority ordering

**Rationale**:
- The existing codebase already uses prefix detection for special expansions (`~`, `$`, `%`)
- URL schemes (`http://`, `https://`) are unambiguous and don't conflict with path prefixes
- This approach requires no changes to the CSV format (backward compatible)
- Detection can be implemented at the start of expansion, routing to different handlers

**Alternatives Considered**:
1. **Metadata column in CSV**: Rejected because it breaks backward compatibility and complicates the file format
2. **URL validation library**: Rejected because it adds dependencies and we don't need validation, just detection
3. **Regex-based detection**: Rejected as overkill; simple `starts_with` checks are sufficient

### 2. Return Type Design

**Question**: How should the API change to support returning strings instead of paths?

**Decision**: Introduce a `Target` enum with `Path(PathBuf)` and `String(String)` variants

**Rationale**:
- Maintains type safety by distinguishing paths (which can be further processed) from raw strings
- The CLI (`main.rs`) can pattern match and output appropriately
- The library API remains explicit about what it returns
- Backward compatible: paths still get full expansion support

**Alternatives Considered**:
1. **Return `String` everywhere**: Rejected because it loses type information; callers can't distinguish paths from other strings
2. **Return `PathBuf` with lossy conversion**: Rejected because URLs/strings forced into PathBuf would be corrupted on Windows (different separators)
3. **Trait-based abstraction**: Rejected as over-engineering for this simple case

### 3. Database Storage Type

**Question**: Should the database store `PathBuf` or `String`?

**Decision**: Change from `HashMap<String, PathBuf>` to `HashMap<String, String>`

**Rationale**:
- Storing as `String` is neutral; type detection happens at lookup time
- CSV parsing naturally produces strings; converting to PathBuf is premature
- Simplifies the db module; it becomes a pure key-value store
- The expansion module handles type detection and conversion

**Alternatives Considered**:
1. **Store a `Target` enum in the database**: Rejected because detection would need to happen at CSV parse time, duplicating logic
2. **Keep `PathBuf` storage**: Rejected because non-path strings would be corrupted

### 4. Output Format for Non-Path Targets

**Question**: How should URLs and arbitrary strings be output?

**Decision**: Output verbatim as UTF-8 bytes to stdout (same as current path behavior)

**Rationale**:
- Current code writes `PathBuf` as bytes via `as_os_str().as_bytes()`
- For strings, we use `.as_bytes()` directly
- No newline appended (consistent with current behavior)
- Shell scripts can capture output as-is

**Alternatives Considered**:
1. **Add newline**: Rejected to maintain backward compatibility
2. **Different output for different types**: Rejected as unnecessary complexity

### 5. Edge Case: Path-like Strings

**Question**: What if a user wants to store a string that looks like a path but shouldn't be expanded?

**Decision**: Defer to future escape mechanism; document current behavior

**Rationale**:
- This is an edge case unlikely to occur in practice
- The spec explicitly states strings starting with `/`, `~`, `$`, `%` are treated as paths
- A future enhancement could add an escape prefix (e.g., `\` or `!` to mean "literal")
- Documenting the behavior is sufficient for now

**Alternatives Considered**:
1. **Implement escape mechanism now**: Rejected as premature; no user need demonstrated
2. **Use metadata field**: Rejected as format-breaking

## Resolved NEEDS CLARIFICATION

No items required clarification. All technical decisions were derivable from the spec and existing codebase patterns.

## Implementation Notes

### Detection Order (Priority)
1. Check for `http://` or `https://` prefix → URL (verbatim output)
2. Check for `/`, `~`, `$`, or `%` prefix → Path (apply expansion)
3. All other strings → Arbitrary (verbatim output)

### Code Changes Summary
| File | Change |
|------|--------|
| `db.rs` | `HashMap<String, PathBuf>` → `HashMap<String, String>` |
| `lib.rs` | `App::path()` → `App::resolve()` returning `Target` enum |
| `expansion.rs` | Implement `Expand::resolve()` with type detection logic |
| `main.rs` | Handle `Target` enum in output |

### Backward Compatibility
- Existing `targets.csv` files work without modification
- All current path targets continue to expand correctly
- The CSV format is unchanged: `value,name1,name2,...`
