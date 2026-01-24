# Zellij Keybindings Reference

Default mode is **locked** (keystrokes pass through to terminal). Press `Ctrl+g` to enter normal mode.

## Always Available (including locked mode)

These bindings work in locked mode, your primary mode for interacting with the terminal.

| Key | Description |
|-----|-------------|
| `Alt+h` | Focus pane to the left, or previous tab if at edge |
| `Alt+j` | Focus pane below |
| `Alt+k` | Focus pane above |
| `Alt+l` | Focus pane to the right, or next tab if at edge |
| `Alt+n` | Cycle focus to next pane |
| `Alt+,` | Go to previous tab |
| `Alt+.` | Go to next tab |
| `Alt+z` | Toggle pane border frames on/off |

## Unlocking: Locked → Normal

| Key | Description |
|-----|-------------|
| `Ctrl+g` | Enter normal mode (unlocks keybindings) |

## Normal Mode

From normal mode, you can switch to specialized modes or perform quick actions.

### Mode Switching

| Key | Description |
|-----|-------------|
| `g` | Send literal Ctrl+g to terminal, return to locked |
| `Ctrl+g` | Return to locked |
| `Ctrl+p` | Enter **Pane** mode |
| `Ctrl+t` | Enter **Tab** mode |
| `Ctrl+n` | Enter **Resize** mode |
| `Ctrl+s` | Enter **Scroll** mode |
| `Ctrl+o` | Enter **Session** mode |
| `Ctrl+h` | Enter **Move** mode (rearrange panes) |
| `Ctrl+b` | Enter **Tmux** mode (tmux-style bindings) |
| `Ctrl+q` | Quit Zellij |

### Quick Actions (all unlocked modes)

These work in normal and other unlocked modes:

| Key | Description |
|-----|-------------|
| `Alt+f` | Toggle floating pane layer visibility |
| `Alt+n` | Create new pane in best available space |
| `Alt+h` / `Alt+Left` | Focus left pane or previous tab |
| `Alt+l` / `Alt+Right` | Focus right pane or next tab |
| `Alt+j` / `Alt+Down` | Focus pane below |
| `Alt+k` / `Alt+Up` | Focus pane above |
| `Alt+i` | Move current tab left in tab bar |
| `Alt+o` | Move current tab right in tab bar |
| `Alt+=` / `Alt++` | Increase size of focused pane |
| `Alt+-` | Decrease size of focused pane |
| `Alt+[` | Switch to previous layout variant |
| `Alt+]` | Switch to next layout variant |
| `Alt+p` | Toggle pane in/out of current group |
| `Alt+Shift+p` | Toggle group marking mode |
| `Enter` / `Esc` | Return to normal mode (from other modes) |

---

## Pane Mode (`Ctrl+g` → `Ctrl+p`)

Manage pane creation, navigation, and properties.

| Key | Description |
|-----|-------------|
| `Ctrl+p` | Return to normal mode |
| `h` / `Left` | Focus pane to the left |
| `j` / `Down` | Focus pane below |
| `k` / `Up` | Focus pane above |
| `l` / `Right` | Focus pane to the right |
| `p` | Toggle focus between current and previous pane |
| `n` | Create new pane in best available space |
| `d` | Create new pane below (split down) |
| `r` | Create new pane to the right (split right) |
| `s` | Create new stacked pane |
| `x` | Close focused pane |
| `f` | Toggle fullscreen zoom on focused pane |
| `z` | Toggle pane border frames globally |
| `w` | Toggle floating pane layer visibility |
| `e` | Convert floating pane to tiled, or vice versa |
| `c` | Rename focused pane |
| `i` | Pin/unpin pane (prevents auto-closing) |

---

## Tab Mode (`Ctrl+g` → `Ctrl+t`)

Manage tabs and move panes between them.

| Key | Description |
|-----|-------------|
| `Ctrl+t` | Return to normal mode |
| `h` / `Left` / `k` / `Up` | Go to previous tab |
| `l` / `Right` / `j` / `Down` | Go to next tab |
| `n` | Create new tab |
| `x` | Close current tab |
| `r` | Rename current tab |
| `Tab` | Toggle between current and last-used tab |
| `1`-`9` | Jump to tab by number |
| `s` | Sync mode: send keystrokes to all panes in tab |
| `b` | Break focused pane into new tab |
| `]` | Break focused pane into new tab to the right |
| `[` | Break focused pane into new tab to the left |

---

## Resize Mode (`Ctrl+g` → `Ctrl+n`)

Resize the focused pane.

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
| `=` / `+` | Increase pane size equally in all directions |
| `-` | Decrease pane size equally in all directions |

---

## Move Mode (`Ctrl+g` → `Ctrl+h`)

Physically relocate panes within the layout.

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

Scroll through terminal history and search.

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
| `n` | Jump to next match |
| `p` | Jump to previous match |
| `c` | Toggle case sensitivity |
| `w` | Toggle wrap-around search |
| `o` | Toggle whole-word matching |
| `j` / `k` | Scroll while searching |
| `d` / `u` | Half-page scroll while searching |
| `Ctrl+s` | Return to normal mode |

---

## Session Mode (`Ctrl+g` → `Ctrl+o`)

Manage the Zellij session and plugins.

| Key | Description |
|-----|-------------|
| `Ctrl+o` | Return to normal mode |
| `Ctrl+s` | Enter scroll mode |
| `d` | Detach from session (session keeps running) |
| `w` | Open session manager (switch/create sessions) |
| `c` | Open configuration plugin |
| `p` | Open plugin manager |
| `a` | Open about dialog |
| `s` | Open sharing settings |

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
| `c` | Create new tab |
| `,` | Rename current tab |
| `p` | Go to previous tab |
| `n` | Go to next tab |
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

### Rename Tab (from Tab mode: `r`)

| Key | Description |
|-----|-------------|
| *type name* | Enter new tab name |
| `Ctrl+c` | Confirm and return to normal mode |
| `Esc` | Cancel rename, return to tab mode |

### Rename Pane (from Pane mode: `c`)

| Key | Description |
|-----|-------------|
| *type name* | Enter new pane name |
| `Ctrl+c` | Confirm and return to normal mode |
| `Esc` | Cancel rename, return to pane mode |
