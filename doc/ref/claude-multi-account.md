# Multiple Claude Accounts on One Machine

Claude Code stores auth, settings, sessions, and history under a single config
directory — `~/.claude` by default. Two accounts (e.g. home and work) can
coexist by giving each its own config directory.

## Separate config dirs (recommended)

`CLAUDE_CONFIG_DIR` overrides the default location:

```sh
CLAUDE_CONFIG_DIR=~/.claude-home claude
CLAUDE_CONFIG_DIR=~/.claude-work claude
```

Run `claude auth login` once inside each to authenticate that directory. After
that, credentials, settings, MCP servers, and session history stay fully
separate.

Wrap in shell functions so the var is never forgotten:

```sh
claude-home() { CLAUDE_CONFIG_DIR=~/.claude-home claude $@ }
claude-work() { CLAUDE_CONFIG_DIR=~/.claude-work claude $@ }
```

## Logout / login each time

`claude auth logout` then `claude auth login` also works, but re-authenticates
on every switch and shares one set of settings and history. Only worth it if
switching is rare.

## Relevant commands

| Command                    | Purpose                                    |
| -------------------------- | ------------------------------------------ |
| `claude auth login`        | Sign in (`--claudeai` subscription default) |
| `claude auth login --sso`  | Force SSO flow — likely needed for work     |
| `claude auth login --console` | Anthropic Console / API billing instead  |
| `claude auth login --email <addr>` | Pre-populate the login page        |
| `claude auth status --text`  | Show which account is active             |
| `claude auth logout`       | Sign out of the active config dir           |

`claude auth status` is useful for confirming the config dir override actually
took effect.

## Caveats

- Verified against Claude Code 2.1.215. `CLAUDE_CONFIG_DIR` is referenced
  directly in the binary; the flags above come from `--help`.
- Credentials on macOS may also live in the Keychain. If the two accounts
  collide despite separate config dirs, the Keychain is the first place to
  look. Note that `--bare` explicitly skips keychain reads.
- Anything pointing at `~/.claude` by absolute path (hooks, scripts, symlinked
  dotfiles) will not follow the override and needs adjusting.
