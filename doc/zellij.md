# Zellij Cheat Sheet

Default mode is **locked** (keystrokes go to terminal). Press `Ctrl+g` to enter normal mode.

## Quick bindings (work in locked mode)

| Key | Action |
|-----|--------|
| `Alt+h/j/k/l` | Focus pane (h/l also switch tabs) |
| `Alt+n` | Focus next pane |
| `Alt+,` | Previous tab |
| `Alt+.` | Next tab |
| `Alt+t` | New tab |
| `Alt+[` | Previous layout |
| `Alt+]` | Next layout |
| `Alt+z` | Toggle pane frames |

## Quick actions (`Ctrl+g` then key, returns to locked)

| Key | Action |
|-----|--------|
| `Ctrl+g` | Send literal Ctrl+g to app |
| `n` | New pane |
| `d` | Split down |
| `r` | Split right |
| `x` | Close pane |
| `z` | Zoom (fullscreen pane) |
| `f` | Toggle floating panes |
| `t` | New tab |
| `w` | Close tab |

## Default mode switching (from normal mode)

For less common operations, use Zellij's default modes:

| Key | Mode |
|-----|------|
| `Ctrl+g` | Back to locked |
| `Ctrl+p` | Pane |
| `Ctrl+t` | Tab |
| `Ctrl+n` | Resize |
| `Ctrl+s` | Scroll |
| `Ctrl+o` | Session |
| `Ctrl+h` | Move |
| `Ctrl+q` | Quit |

## Pane mode (`Ctrl+g` then `Ctrl+p`)

| Key | Action |
|-----|--------|
| `n` | New pane |
| `d` | Split down |
| `r` | Split right |
| `x` | Close pane |
| `f` | Toggle fullscreen |
| `w` | Toggle floating |
| `e` | Toggle embed/float |
| `c` | Rename pane |
| `h/j/k/l` | Navigate panes |

## Tab mode (`Ctrl+g` then `Ctrl+t`)

| Key | Action |
|-----|--------|
| `n` | New tab |
| `x` | Close tab |
| `r` | Rename tab |
| `h/l` | Previous/next tab |
| `1-9` | Go to tab N |
| `Tab` | Toggle last tab |
| `s` | Sync panes (input to all) |
| `b` | Break pane to new tab |

## Scroll mode (`Ctrl+g` then `Ctrl+s`)

| Key | Action |
|-----|--------|
| `j/k` | Scroll down/up |
| `d/u` | Half page down/up |
| `Ctrl+f/b` | Page down/up |
| `s` | Search |
| `e` | Edit scrollback in $EDITOR |

## Session mode (`Ctrl+g` then `Ctrl+o`)

| Key | Action |
|-----|--------|
| `d` | Detach |
| `w` | Session manager |
