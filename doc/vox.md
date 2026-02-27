# Virtual Environments in Xonsh

## The core problem

Xonsh **is** a Python interpreter. When you activate a traditional venv (the
`source .venv/bin/activate` dance), the activate script is a shell-specific
hack — there are versions for bash, zsh, fish, etc., but **not for xonsh**. So
standard `venv`/`virtualenv` activation simply doesn't work.

This also means `uv` won't "just work" for activation. `uv venv` can *create*
a venv fine, and `uv run` executes commands in the venv without needing
activation, but `source .venv/bin/activate` will fail in xonsh.

## What xonsh provides instead

Xonsh ships **Vox**, its own venv manager (`xpip install xontrib-vox`). Key
commands:

- `vox new <name>` — create a venv (stored in `$VIRTUALENV_HOME`, default `~/.virtualenvs`)
- `vox activate <name>` — activate it (also `workon`)
- `vox deactivate` — deactivate
- `vox list` / `vox remove`

You can also activate a venv by path: `vox activate /path/to/.venv`.

## If you want uv specifically

There's a community xontrib called [xontrib-uvox](https://github.com/LoicGrobol/xontrib-uvox)
that replaces Vox's backend with `uv` — so you get Vox-style commands but `uv`
does the actual venv creation/management.

## Practical workarounds without xontribs

- **`uv run <command>`** — runs inside the project's venv without activation.
  This sidesteps the whole problem.
- **`vox activate .venv`** — if you already created a venv with `uv venv`, you
  can point Vox at it.
- For `uv pip install` targeting xonsh's own environment, you may need
  `--python` to point at the right interpreter.

## Quick reference

| Operation | Works in xonsh? |
|---|---|
| `uv venv` (create) | Yes |
| `source .venv/bin/activate` | No |
| `uv run <cmd>` | Yes |
| `vox activate .venv` | Yes (with xontrib-vox) |

The safest pattern is: create venvs however you like (`uv venv`,
`python -m venv`, etc.), but activate them through **Vox** or skip activation
entirely with **`uv run`**.

## Sources

- [Xonsh Virtual Environments docs](https://xon.sh/python_virtual_environments.html)
- [xontrib-uvox](https://github.com/LoicGrobol/xontrib-uvox)
- [uv issue #4635 — xonsh-specific pip targeting](https://github.com/astral-sh/uv/issues/4635)
