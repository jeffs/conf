# Data Model: Generalize Jump Targets

**Feature Branch**: `002-generalize-jump-targets`
**Date**: 2025-12-13

## Entities

### Target (New)

Represents the resolved value of a jump target lookup.

```rust
pub enum Target {
    /// A filesystem path (expanded from `~`, `$VAR`, `%date`, or absolute paths)
    Path(PathBuf),
    /// A URL or arbitrary string (output verbatim)
    String(String),
}
```

**Attributes**:
- `Path(PathBuf)`: Expanded filesystem path; suitable for `cd` or file operations
- `String(String)`: Raw string value; suitable for URLs, IDs, or any other text

**Validation Rules**:
- `Path`: Must be non-empty after expansion
- `String`: No validation; stored value is output as-is

**State Transitions**: N/A (immutable value type)

### Database Entry (Modified)

The database maps short names to raw target values.

**Before**:
```rust
HashMap<String, PathBuf>  // name → path
```

**After**:
```rust
HashMap<String, String>   // name → value (type determined at resolve time)
```

**Attributes**:
- Key: Short name (e.g., `prj`, `gh-repo`, `current-issue`)
- Value: Raw target string from CSV (e.g., `~/projects`, `https://github.com/...`, `#456`)

**Validation Rules**:
- Key: Non-empty string, no commas (CSV constraint)
- Value: Non-empty string

## Relationships

```text
┌─────────────┐     lookup      ┌──────────────┐     resolve     ┌──────────┐
│  Database   │ ──────────────► │  Raw Value   │ ─────────────►  │  Target  │
│  (CSV file) │                 │  (String)    │                 │  (Enum)  │
└─────────────┘                 └──────────────┘                 └──────────┘
      │                                │                               │
      │                                │                               │
      ▼                                ▼                               ▼
  name → value               type detection              Path(expanded) or
  "prj" → "~/conf"          by prefix/scheme            String(verbatim)
```

## Type Detection Logic

```text
Input: raw value string from database

1. Does value start with "http://" or "https://"?
   └─ YES → Target::String(value)  [URL - verbatim]

2. Does value start with "/" or "~" or "$" or "%"?
   └─ YES → Target::Path(expand(value))  [Path - apply expansion]

3. Otherwise
   └─ Target::String(value)  [Arbitrary - verbatim]
```

## CSV Format

**Unchanged from current format**:

```csv
# Comment lines start with #
# Blank lines are ignored

# Format: value,name1,name2,...

# Paths (expanded)
~/projects,prj
$HOME/documents,docs
/var/log/%Y-%m-%d,today-logs

# URLs (verbatim)
https://github.com/user/repo,gh-repo,repo
http://localhost:3000,local

# Arbitrary strings (verbatim)
#456,current-issue
PR-789,current-pr
user@host:path,ssh-prod
```

## Error States

| Condition | Error Type | Message |
|-----------|------------|---------|
| Target name not in database | `Error::Target(name)` | `{name}: not found` |
| Path expansion results in empty | `expansion::Error::Empty` | `Empty target` |
| Environment variable unset | `expansion::Error::Unset` | `Unset variable` |
| CSV syntax error | `db::Error::Syntax` | `{file}:{line}: Syntax error` |

## Output Format

All target types output to stdout without trailing newline:

| Target Type | Output |
|-------------|--------|
| `Target::Path(p)` | `p.as_os_str().as_bytes()` |
| `Target::String(s)` | `s.as_bytes()` |
