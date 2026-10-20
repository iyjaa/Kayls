# kayls-editor

Editor widget for Tscode — a terminal-based IDE written in Rust.

## Overview

kayls-editor provides a complete code editor widget for `ratatui`. It handles:

- Text buffer with rope data structure
- Tree-sitter syntax highlighting
- Cursor movement & selection
- Undo/redo history
- Code folding
- Auto-pair (brackets, quotes)
- Auto-indent
- Themes

The widget is rendered via `Widget for &Editor`, and input is handled via `Editor::input()`.

## Architecture

```
src/
├── lib.rs              # Crate root — module declarations
├── editor.rs           # Main Editor struct, state, public API
├── code.rs             # Text buffer (ropey) + tree-sitter integration
├── actions.rs          # Edit operations (Insert, Delete, Move, etc.)
├── view.rs             # Visual row mapping (wrapping, folding, diff)
├── render.rs           # Rendering logic — draws to buffer
├── selection.rs        # Text selection state
├── history.rs          # Undo/redo history
├── theme.rs            # Theme definitions (vesper, gruvbox, dracula, nord)
├── types.rs            # Shared types (VisualRow, Theme, etc.)
├── utils.rs            # Helpers (rgb parsing, grapheme width)
├── click.rs            # Mouse click tracking
├── diff.rs             # Diff computation (for diff view)
├── auto_pair.rs        # Auto-pair logic (Tscode addition)
└── editor_crossterm.rs # Crossterm input handler
```

## Data Flow

```
+---------------------------------------------------------+
|                    Input (crossterm)                     |
|                  KeyEvent, MouseEvent                    |
+----------------------------+----------------------------+
                             |
                             v
+---------------------------------------------------------+
|              Editor::input() / Editor::mouse()           |
|                 (in editor_crossterm.rs)                 |
|                                                          |
|  1. Check auto-pair (try_auto_pair)                     |
|  2. Map key to Action                                    |
|  3. Apply action to editor state                         |
+----------------------------+----------------------------+
                             |
                             v
+---------------------------------------------------------+
|                   Action::apply(&mut Editor)             |
|                    (in actions.rs)                       |
|                                                          |
|  1. Get cursor + selection                               |
|  2. Modify Code buffer (code.rs)                         |
|  3. Update cursor + selection                            |
|  4. Commit history                                       |
+----------------------------+----------------------------+
                             |
                             v
+---------------------------------------------------------+
|                    Editor State Update                   |
|                                                          |
|  - code: Code (text buffer)                              |
|  - cursor: usize                                         |
|  - selection: Option<Selection>                          |
|  - offset_x, offset_y: scroll                            |
|  - history: Undo/Redo                                    |
+----------------------------+----------------------------+
                             |
                             v
+---------------------------------------------------------+
|              Widget::render(&Editor, area, buf)          |
|                    (in render.rs)                        |
|                                                          |
|  1. Compute visible rows (view.rs)                       |
|  2. For each row:                                        |
|     a. Draw line number (if enabled)                     |
|     b. Draw syntax highlights                            |
|     c. Draw selection                                    |
|     d. Draw cursor                                       |
+----------------------------+----------------------------+
                             |
                             v
+---------------------------------------------------------+
|                  Terminal (ratatui buffer)               |
+---------------------------------------------------------+
```

## Key Components

### Editor (editor.rs)

The main struct. Holds all editor state.

```rust
pub struct Editor {
    pub(crate) code: Code,
    pub(crate) cursor: usize,
    pub(crate) offset_y: usize,
    pub(crate) offset_x: usize,
    pub(crate) theme: Theme,
    pub(crate) selection: Option<Selection>,
    pub(crate) clicks: ClickTracker,
    pub(crate) selection_snap: SelectionSnap,
    pub(crate) clipboard: Option<String>,
    pub(crate) marks: Option<Vec<(usize, usize, Color)>>,
    pub(crate) highlights_cache: RefCell<HightlightCache>,
    pub(crate) show_line_numbers: bool,
    pub(crate) code_folding_options: CodeFoldingOptions,
    pub(crate) view: View,
}
```

Public API:

| Method | Purpose |
|--------|---------|
| `new(lang, text, theme)` | Create editor |
| `input(key, area)` | Handle keyboard input |
| `mouse(event, area)` | Handle mouse input |
| `get_content()` | Get text content |
| `set_content(text)` | Set text content |
| `get_cursor()` | Get cursor position |
| `set_cursor(idx)` | Set cursor position |
| `get_visible_cursor(area)` | Get screen position for cursor |
| `show_line_numbers(bool)` | Toggle line numbers |
| `focus(area)` | Ensure cursor visible |

### Code (code.rs)

Text buffer wrapper around `ropey::Rope` plus tree-sitter integration.

```rust
pub(crate) struct Code {
    rope: Rope,
    lang: String,
    tree: Option<Tree>,
    parser: Option<Parser>,
    highlight_query: Option<Query>,
    fold_query: Option<Query>,
}
```

Key methods:

| Method | Purpose |
|--------|---------|
| `new(text, lang, custom)` | Create from text |
| `insert(from, text)` | Insert text at position |
| `remove(start, end)` | Remove range |
| `char_slice(start, end)` | Get slice |
| `len_chars()` | Total character count |
| `point(char_idx)` | Char index to (line, col) |
| `line_to_char(line)` | Line index to char index |
| `tx()` / `commit()` | Transaction for history |

### Actions (actions.rs)

All edit operations implement the `Action` trait.

```rust
pub trait Action {
    fn apply(&mut self, editor: &mut Editor);
}
```

Built-in actions:

| Action | Purpose |
|--------|---------|
| `InsertText { text }` | Insert string |
| `InsertNewline` | New line + auto-indent |
| `Delete` | Backspace |
| `MoveLeft/Right/Up/Down` | Cursor movement |
| `Indent` / `UnIndent` | Tab / Shift+Tab |
| `Undo` / `Redo` | History |
| `Copy` / `Paste` / `Cut` | Clipboard |
| `SelectAll` | Selection |
| `Duplicate` | Duplicate line |
| `DeleteLine` | Delete current line |
| `ToggleComment` | Comment/uncomment |

### View (view.rs)

Maps document lines to visual rows. Handles:

- Line wrapping
- Code folding (collapse ranges)
- Diff view (added/deleted ghost lines)

### Selection (selection.rs)

Text selection with two modes:

- `SelectionSnap::None` — free selection
- `SelectionSnap::Word` — snap to word
- `SelectionSnap::Line` — snap to line

### History (history.rs)

Undo/redo stack. Tracks cursor + selection for each edit.

### Theme (theme.rs)

Theme mapping: `HashMap<String, Style>` — token name to style.

Built-in themes:

| Theme | Style |
|-------|-------|
| `vesper()` | Green, orange, purple (default) |
| `gruvbox()` | Warm, retro |
| `dracula()` | Dark, soft pink |
| `nord()` | Cool, blue |

## Auto-pair & Auto-indent (Tscode additions)

### Auto-pair (auto_pair.rs)

When user types an opening character, the matching closing character is inserted automatically.

```rust
pub fn try_auto_pair(editor: &mut Editor, c: char) -> bool {
    // Returns true if handled
}
```

Supported pairs:

| Type | Insert |
|------|--------|
| `(` | `()` |
| `{` | `{}` |
| `[` | `[]` |
| `<` | `<>` |
| `"` | `""` |
| `'` | `''` |
| `` ` `` | `` ` ` `` |

Skip closing: if closing char already ahead, cursor just moves forward.

### Auto-indent (actions.rs)

`InsertNewline::apply` handles:

- Enter after `{`, `(`, `[` — indent +1
- Enter inside `{|}` — moves `}` to new line

## Themes

Change theme via `Editor::new(lang, text, theme)`:

```rust
use kayls_editor::editor::Editor;
use kayls_editor::theme::{vesper, gruvbox, dracula, nord};

let editor = Editor::new("rust", "fn main() {}", vesper())?;
let editor = Editor::new("rust", "fn main() {}", dracula())?;
```

## How to Modify

### Adding a new action

1. Open `src/actions.rs`
2. Add struct:
   ```rust
   pub struct MyAction;
   impl Action for MyAction {
       fn apply(&mut self, editor: &mut Editor) {
           // modify editor state
       }
   }
   ```
3. Register in `editor_crossterm.rs`:
   ```rust
   KeyCode::Char('x') => self.apply(MyAction {}),
   ```

### Adding a new language

1. Add tree-sitter grammar in `Cargo.toml`:
   ```toml
   tree-sitter-my-lang = "~0.1"
   ```
2. Add query files in `langs/my-lang/`:
   - `highlights.scm`
   - `folds.scm` (optional)
3. Register in `code.rs` — `Code::new()`.

### Changing theme colors

1. Open `src/theme.rs`
2. Modify `vesper()`, `gruvbox()`, etc.
3. Or add new function:
   ```rust
   pub fn my_theme() -> Vec<(&'static str, &'static str)> {
       vec![
           ("keyword", "#ff0000"),
           ("string", "#00ff00"),
       ]
   }
   ```

### Adding auto-pair character

1. Open `src/auto_pair.rs`
2. Add to `is_opening`:
   ```rust
   '<' => Some('>'),
   ```
3. Add to `is_closing`:
   ```rust
   matches!(c, ... | '>')
   ```

## License

MIT — see [LICENSE](LICENSE).