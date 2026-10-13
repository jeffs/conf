# File Zellij feature requests

File these with the Zellij maintainers, motivated by running several coding agents, each in a multi-pane tab dedicated to one goal (agent, shell, editor, file watcher, etc.):

1. **Tab names from panes with frames on.** Let a tab's name follow a chosen pane's title, such as the focused pane or one marked in the layout, even when pane frames are enabled and the tab has several panes. Currently this works only with frames disabled, and only for single-pane tabs.
2. **Vertical tab list.** Offer a tab bar that lists tabs vertically, so long titles stay readable. Titles currently appear side by side.
3. **Per-tab pane frames.** Allow `pane_frames` or `pane_frame_style` to be set per tab instead of per session. Per-pane `borderless` is the closest current workaround.
4. **Title updates from Claude Code.** Claude Code stops updating its terminal title (e.g., idle status) when `ZELLIJ` is set, and its full TUI needs compatibility workarounds under Zellij. This may belong with Anthropic instead, unless Zellij can fix the underlying incompatibility.
5. **Direct pane selection.** Offer an overlay like WezTerm's `PaneSelect`, labeling each pane so typing its label focuses it. Currently `zellij action focus-pane-id` can focus a pane, but only if you already know its ID, for example from `zellij action list-panes`.

## See also

- [herdr](https://github.com/herdrdev/herdr), an agent-focused multiplexer that addresses some of these.
