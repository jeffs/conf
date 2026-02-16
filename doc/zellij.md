# Zellij Keybindings Reference

Zellij 0.43.1. Default mode is **locked** (keystrokes pass through to terminal). Press `Ctrl+g` to enter normal mode.

## Locked Mode (default)

All keystrokes pass through to the terminal except:

| Key | Description |
|-----|-------------|
| `Ctrl+g` | Enter normal mode (unlock) |
| `Alt+h` | Focus pane left, or previous tab at edge |
| `Alt+j` | Focus pane below |
| `Alt+k` | Focus pane above |
| `Alt+l` | Focus pane right, or next tab at edge |
| `Alt+n` | Focus next pane |
| `Alt+p` | Focus previous pane |
| `Alt+,` | Previous tab |
| `Alt+.` | Next tab |
| `Alt+t` | New tab |
| `Alt+1`–`9` | Go to tab 1–9 |
| `Alt+0` | Go to tab 10 |
| `Alt+z` | Toggle fullscreen zoom |
| `Alt+Z` | Toggle fullscreen zoom + toggle pane frames |

## Normal Mode (`Ctrl+g`)

### Quick Actions (return to locked)

| Key | Description |
|-----|-------------|
| `g` | Send literal Ctrl+g to terminal |
| `f` | Toggle floating panes |
| `j` | New pane below (split down) |
| `l` | New pane right (split right) |
| `r` | Rename current tab (enters rename mode) |

### Mode Switching

| Key | Description |
|-----|-------------|
| `Ctrl+g` | Return to locked mode |
| `Ctrl+p` | Enter **Pane** mode |
| `Ctrl+t` | Enter **Tab** mode |
| `Ctrl+n` | Enter **Resize** mode |
| `Ctrl+s` | Enter **Scroll** mode |
| `Ctrl+o` | Enter **Session** mode |
| `Ctrl+h` | Enter **Move** mode |
| `Ctrl+b` | Enter **Tmux** mode |
| `Ctrl+q` | Quit Zellij |

## Shared Across Unlocked Modes

Available in normal mode and all sub-modes, but **not** locked mode:

| Key | Description |
|-----|-------------|
| `Alt+f` | Toggle floating pane layer |
| `Alt+n` | New pane (note: overridden to focus-next in normal) |
| `Alt+h` / `Alt+Left` | Focus pane left or previous tab |
| `Alt+l` / `Alt+Right` | Focus pane right or next tab |
| `Alt+j` / `Alt+Down` | Focus pane below |
| `Alt+k` / `Alt+Up` | Focus pane above |
| `Alt+i` | Move current tab left in tab bar |
| `Alt+o` | Move current tab right in tab bar |
| `Alt+=` / `Alt++` | Increase pane size |
| `Alt+-` | Decrease pane size |
| `Alt+[` | Previous swap layout |
| `Alt+]` | Next swap layout |
| `Alt+p` | Toggle pane in/out of group (note: overridden to focus-prev in normal) |
| `Alt+Shift+p` | Toggle group marking mode |
| `Enter` / `Esc` | Return to normal mode (from sub-modes) |

---

## Pane Mode (`Ctrl+g` → `Ctrl+p`)

| Key | Description |
|-----|-------------|
| `Ctrl+p` | Return to normal mode |
| `h` / `Left` | Focus pane left |
| `j` / `Down` | Focus pane below |
| `k` / `Up` | Focus pane above |
| `l` / `Right` | Focus pane right |
| `p` | Toggle focus between current and previous pane |
| `n` | New pane in best available space |
| `d` | New pane below (split down) |
| `r` | New pane to the right (split right) |
| `s` | New stacked pane |
| `x` | Close focused pane |
| `f` | Toggle fullscreen zoom on focused pane |
| `z` | Toggle pane border frames globally |
| `w` | Toggle floating pane layer |
| `e` | Convert floating pane to tiled, or vice versa |
| `c` | Rename focused pane |
| `i` | Pin/unpin pane |

---

## Tab Mode (`Ctrl+g` → `Ctrl+t`)

| Key | Description |
|-----|-------------|
| `Ctrl+t` | Return to normal mode |
| `h` / `Left` / `k` / `Up` | Previous tab |
| `l` / `Right` / `j` / `Down` | Next tab |
| `n` | New tab |
| `x` | Close current tab |
| `r` | Rename current tab |
| `Tab` | Toggle between current and last-used tab |
| `1`–`9` | Jump to tab by number |
| `s` | Sync mode: send keystrokes to all panes in tab |
| `b` | Break focused pane into new tab |
| `]` | Break focused pane into new tab to the right |
| `[` | Break focused pane into new tab to the left |

---

## Resize Mode (`Ctrl+g` → `Ctrl+n`)

| Key | Description |
|-----|-------------|
| `Ctrl+n` | Return to normal mode |
| `h` / `Left` | Grow pane leftward |
| `j` / `Down` | Grow pane downward |
| `k` / `Up` | Grow pane upward |
| `l` / `Right` | Grow pane rightward |
| `H` | Shrink pane from left edge |
| `J` | Shrink pane from bottom edge |
| `K` | Shrink pane from top edge |
| `L` | Shrink pane from right edge |
| `=` / `+` | Increase pane size equally |
| `-` | Decrease pane size equally |

---

## Move Mode (`Ctrl+g` → `Ctrl+h`)

| Key | Description |
|-----|-------------|
| `Ctrl+h` | Return to normal mode |
| `n` / `Tab` | Swap pane with next pane |
| `p` | Swap pane with previous pane |
| `h` / `Left` | Move pane left |
| `j` / `Down` | Move pane down |
| `k` / `Up` | Move pane up |
| `l` / `Right` | Move pane right |

---

## Scroll Mode (`Ctrl+g` → `Ctrl+s`)

| Key | Description |
|-----|-------------|
| `Ctrl+s` | Return to normal mode |
| `j` / `Down` | Scroll down one line |
| `k` / `Up` | Scroll up one line |
| `d` | Scroll down half a page |
| `u` | Scroll up half a page |
| `Ctrl+f` / `PageDown` / `l` | Scroll down one page |
| `Ctrl+b` / `PageUp` / `h` | Scroll up one page |
| `Ctrl+c` | Jump to bottom and return to normal mode |
| `s` | Enter search mode |
| `e` | Open scrollback in $EDITOR |

### Search Mode (from scroll: `s`)

| Key | Description |
|-----|-------------|
| *type query* | Enter search text |
| `Enter` | Confirm search, enter search navigation |
| `Ctrl+c` / `Esc` | Cancel search, return to scroll mode |

### Search Navigation (after Enter)

| Key | Description |
|-----|-------------|
| `n` | Next match |
| `p` | Previous match |
| `c` | Toggle case sensitivity |
| `w` | Toggle wrap-around search |
| `o` | Toggle whole-word matching |
| `j` / `k` | Scroll while searching |
| `d` / `u` | Half-page scroll while searching |
| `Ctrl+s` | Return to normal mode |

---

## Session Mode (`Ctrl+g` → `Ctrl+o`)

| Key | Description |
|-----|-------------|
| `Ctrl+o` | Return to normal mode |
| `Ctrl+s` | Enter scroll mode |
| `d` | Detach from session (session keeps running) |
| `w` | Session manager (switch/create sessions) |
| `c` | Configuration plugin |
| `p` | Plugin manager |
| `a` | About dialog |
| `s` | Sharing settings |

---

## Tmux Mode (`Ctrl+g` → `Ctrl+b`)

Tmux-compatible bindings for muscle memory.

| Key | Description |
|-----|-------------|
| `Ctrl+b` | Send literal Ctrl+b to terminal, return to normal |
| `[` | Enter scroll mode |
| `"` | Split pane horizontally (new pane below) |
| `%` | Split pane vertically (new pane right) |
| `z` | Toggle fullscreen zoom |
| `c` | New tab |
| `,` | Rename current tab |
| `p` | Previous tab |
| `n` | Next tab |
| `h` / `Left` | Focus pane left |
| `j` / `Down` | Focus pane down |
| `k` / `Up` | Focus pane up |
| `l` / `Right` | Focus pane right |
| `o` | Cycle focus to next pane |
| `d` | Detach from session |
| `Space` | Cycle to next layout |
| `x` | Close focused pane |

---

## Rename Modes

### Rename Tab (Normal: `r`, or Tab mode: `r`)

| Key | Description |
|-----|-------------|
| *type name* | Enter new tab name |
| `Ctrl+c` | Confirm and return to normal mode |
| `Esc` | Cancel rename, return to tab mode |

### Rename Pane (Pane mode: `c`)

| Key | Description |
|-----|-------------|
| *type name* | Enter new pane name |
| `Ctrl+c` | Confirm and return to normal mode |
| `Esc` | Cancel rename, return to pane mode |
