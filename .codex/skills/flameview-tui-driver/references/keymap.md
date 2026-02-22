# Flameview TUI Keymap

Use this map while running `flameview <folded-file>` interactively.

## Navigation

- `j` / `k`: move selection down/up
- `PageDown` / `PageUp`: jump by viewport
- `g` / `G`: jump to top/bottom

## Expand/Collapse

- `l` or `Enter`: expand selected row
- `h`: collapse selected row or move to parent
- `space`: toggle selected row
- `a`: expand currently visible expandable rows
- `z`: rerun heuristic auto-expand to fill viewport

## Filters and Search

- `/`: enter search mode
- `Enter` in search mode: apply search and auto-expand
- `Esc` in search mode: cancel search mode
- `n` / `p`: move to next/previous search match
- `[` / `]`: decrease/increase min total-percent filter
- `H`: toggle hot-only filter (`self% >= 1.0`)
- `f`: toggle runtime-frame hiding (`std/core/alloc/libc/startup` frames)
- `r`: reset filters and sort mode

## Sort and Help

- `s`: cycle sort mode (`total`, `self`, `name`)
- `?`: toggle help footer
- `q`: quit
