# Claude Code Agent Monitoring Reference

Claude Code 2.1.221. `claude agents` lists every active session on this machine — interactive terminal sessions and background agents — with live working/waiting status. Docs: <https://code.claude.com/docs/en/agent-view>

## Commands

| Command | Description |
|---------|-------------|
| `claude agents` | Full-screen TUI of all sessions, grouped by state |
| `claude agents --json` | Print sessions as a JSON array and exit (no TTY needed) |
| `claude agents --json --all` | Also include completed background sessions |
| `claude agents --cwd <path>` | Only sessions started under `<path>` |
| `claude logs <id>` | Print a background session's recent terminal output |
| `claude attach <id>` | Attach to a background session |
| `claude stop <id>` | Stop a background session; conversation kept for `claude attach` |
| `claude daemon status` | Show supervisor daemon pid, version, uptime |
| `claude daemon logs` | Tail the daemon log |

`<id>` is the short background-session id from the JSON (`id` field), e.g. `6da9d2f2`.

## TUI Keys

| Key | Description |
|-----|-------------|
| `↑` / `↓` | Move between sessions |
| `Space` | Peek at recent output without attaching |
| `Enter` | Attach to selected session |
| `Ctrl+S` | Toggle grouping: by state vs. by directory |
| `?` | Show all shortcuts |

States shown: **Working**, **Needs input**, **Idle**, **Completed**, **Failed**, **Stopped**. The terminal tab title shows a count of sessions awaiting input.

## JSON Fields

| Field | Description |
|-------|-------------|
| `kind` | `interactive` or `background` |
| `status` | `busy` (working) or `idle` (waiting) — all sessions |
| `state` | Background only: `working`, `blocked`, `done`, `failed`, `stopped` |
| `waitingFor` | Background only, when blocked: what it's waiting on |
| `name` | Session name (interactive: e.g. `conf-75`; background: task summary) |
| `id` | Short id (background only) for `logs` / `attach` / `stop` |
| `sessionId` | Full UUID, usable with `claude --resume` |
| `pid`, `cwd` | Process id and working directory |
| `startedAt` | Unix milliseconds |

Interactive sessions have no `state`; use `status` for them. An interactive `idle` session is waiting for your input.

## Naming Sessions

`name` defaults to directory + suffix for interactive sessions (`conf-65`) and a
Haiku-generated label for background sessions. The descriptive activity line in
the TUI is a separate Haiku-generated summary, not exposed in `--json` — so
name sessions yourself if you want meaningful JSON:

| Command | Description |
|---------|-------------|
| `claude -n <name>` | Set a display name at launch (alias `--name`) |
| `claude --bg --name <name> "<prompt>"` | Name a background agent at dispatch |
| `/rename` | Rename the current session from within it (persists) |
| `Ctrl+R` | Rename the selected session in the `claude agents` TUI |

## One-Liners

Status of everything:

```sh
claude agents --json | jq -r '.[] | [.state // .status, .kind, .name] | @tsv'
```

Only sessions actively working:

```sh
claude agents --json | jq '.[] | select(.status == "busy")'
```

Poll as a crude dashboard:

```sh
while true; do clear; claude agents --json | jq -r '.[] | [.state // .status, .name] | @tsv'; sleep 5; done
```

## Within a Session

| Command | Description |
|---------|-------------|
| `/tasks` | List the current session's background shells and subagents |
| `Ctrl+T` | Toggle the task checklist in the status area |
