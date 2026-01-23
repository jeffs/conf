> [!note]
> Removed from CLAUDE.md pending rust-kart task-mcp bug fix.

## Process Management

Use the MCP task server for background processes, not raw bash backgrounding
or pkill. This includes dev servers, watch processes, and any long-running
commands.

- `task_ensure` to start (idempotent - safe to call if already running)
- `task_list` to see what's running
- `task_logs` to check output
- `task_stop` to stop cleanly

Never use `pkill`, `kill`, or `&` backgrounding for process management.
