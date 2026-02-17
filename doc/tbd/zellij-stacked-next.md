# FocusNextPane / FocusPreviousPane interleave stacked panes with adjacent panes

## Summary

`FocusNextPane` and `FocusPreviousPane` interleave stacked panes with
non-stacked panes (and with panes from other stacks) instead of cycling through
each stack contiguously. This causes counterintuitive focus movement in layouts
that combine stacked and unstacked panes.

## Steps to reproduce

1. Create a layout with a 2-pane stack on the left and an unstacked pane on the
   right:

        [A title]   | [X content]
        [B content]  |

2. Focus pane B (bottom of left stack, expanded).
3. Trigger `FocusNextPane`.
4. Focus moves to A (up, within the stack) instead of X (right, exiting the
   stack).

The pane cycle order is A → X → B → A, interleaving the stack. Expected order
is A → B → X → A — the stack should be grouped.

With two adjacent stacks (left: A,B; right: C,D), the cycle is A → C → B → D
(zigzagging) instead of A → B → C → D.

## Root cause

`next_selectable_pane_id` in `tiled_pane_grid.rs` sorts all panes by `y()`
ascending, breaking ties by `x()`:

```rust
panes.sort_by(|(_a_id, a_pane), (_b_id, b_pane)| {
    if a_pane.y() == b_pane.y() {
        a_pane.x().cmp(&b_pane.x())
    } else {
        a_pane.y().cmp(&b_pane.y())
    }
});
```

This treats every pane as an independent item. Collapsed title bars (y=0, y=1,
etc.) from different stacks or non-stacked panes at y=0 sort interleaved
because they share y-coordinates:

    A(y=0, x=0)  X(y=0, x=50)  B(y=1, x=0)

## Fix (implemented in fork)

Sort stacked panes by `(stack_top_y, stack_x, pane_y)` so all panes in a stack
are contiguous and ordered top-to-bottom within the group. Non-stacked panes
sort by `(pane_y, pane_x, 0)`.

Two helper functions in `tiled_pane_grid.rs`:

- `compute_stack_tops` — for each stack_id, finds the minimum y and the shared
  x across all panes in the stack.
- `cycle_sort_key` — returns the 3-tuple sort key for a pane given the
  precomputed stack tops.

Both `next_selectable_pane_id` and `previous_selectable_pane_id` use the new
sort. No changes to `focus_next_pane` / `focus_previous_pane` in `mod.rs`.

## Relevant code

- `zellij-server/src/panes/tiled_panes/tiled_pane_grid.rs`
  `next_selectable_pane_id` / `previous_selectable_pane_id` — sort and cycle
- `zellij-server/src/panes/tiled_panes/stacked_panes.rs`
  `positions_in_stack()` — stable logical ordering within a stack
- `zellij-server/src/panes/tiled_panes/mod.rs`
  `focus_next_pane` / `focus_previous_pane` — dispatch and `expand_pane` call

## What the fix does NOT change

With a 2-pane stack and no other panes, `FocusNextPane` from the bottom pane
still wraps to the top pane (upward). This is the standard cycle-wrap behavior
(last → first) and is mathematically unavoidable: both next and previous go to
the same pane when there are only two, and the direction is determined by
physical position.

`MoveFocus "up"` / `MoveFocus "down"` remain the correct tool for directional
navigation within stacks — they use spatial position, not index cycling.

## Status

- Fix implemented and tested on a fork based on v0.43.1.
- All 42 stacked-pane tests and 54 focus tests pass.
- No existing upstream issue found as of 2026-02-17.
