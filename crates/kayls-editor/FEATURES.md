# Editor Core Integration Contract

This document defines the public API surface of kayls-editor. The ui-integration team relies on these methods.

**Rule:** Any addition or change to a public method in this crate must be documented here before merging.

## 1. Core State (Read-Only)

| Method | Returns | Description |
|---|---|---|
| `len_chars()` | `usize` | Total characters in the buffer. |
| `line_len(line)` | `usize` | Length of a specific line. |
| `char_to_line(idx)` | `usize` | Convert global char index to line number. |
| `line_to_char(line)` | `usize` | Convert line number to global char index. |
| `get_content()` | `String` | Full buffer as a string. Allocates. |
| `get_cursor()` | `usize` | Current cursor position (char index). |
| `get_offset_x()` | `usize` | Horizontal scroll offset. |
| `get_offset_y()` | `usize` | Vertical scroll offset. |

## 2. State Mutation (Write)

| Method | Parameters | Description |
|---|---|---|
| `set_cursor(idx)` | `usize` | Move cursor to char index. Clamps to bounds. |
| `set_offset_x(x)` | `usize` | Set horizontal scroll. |
| `scroll_up()` | None | Scroll viewport up by 1 line. Does not move cursor. |
| `scroll_down(h)` | `usize` | Scroll viewport down. h is viewport height. |
| `scroll_to_cursor(h)` | `usize` | Adjust scroll so cursor is visible. |

## 3. Input Handling

Pass raw input events to the editor. The editor handles insert, delete, move, and selection.

**Note:** `editor.input()` internally calls `focus()`, which adjusts `offset_x`/`offset_y`. To bypass cursor movement (e.g., Read Mode), intercept arrow keys in the UI and call `scroll_up()`/`scroll_down()` directly.
