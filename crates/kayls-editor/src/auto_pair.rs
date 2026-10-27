//! Auto-pair functionality for the editor.
//!
//! Automatically inserts matching closing characters when typing
//! opening brackets or quotes.

use crate::editor::Editor;

/// Check if character is an opening pair.
fn is_opening(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        '"' => Some('"'),
        '\'' => Some('\''),
        '`' => Some('`'),
        _ => None,
    }
}

/// Check if character is a closing pair.
fn is_closing(c: char) -> bool {
    matches!(c, ')' | ']' | '}' | '>' | '"' | '\'' | '`')
}

/// Get character at cursor position.
fn char_at_cursor(editor: &Editor) -> Option<char> {
    let cursor = editor.get_cursor();
    let code = editor.code_ref();
    let total = code.len_chars();

    if cursor >= total {
        return None;
    }

    let slice = code.char_slice(cursor, cursor + 1);
    slice.chars().next()
}

/// Insert auto-pair at cursor.
///
/// Returns `true` if auto-pair was handled, `false` otherwise.
pub fn try_auto_pair(editor: &mut Editor, c: char) -> bool {
    // Case 1: Opening character — insert pair
    if let Some(closing) = is_opening(c) {
        // Special case: quotes — check if closing quote already ahead
        if c == closing
            && let Some(next) = char_at_cursor(editor)
                && next == closing {
                    // Skip — just move cursor forward
                    let cursor = editor.get_cursor();
                    editor.set_cursor(cursor + 1);
                    return true;
                }

        // Insert pair
        let cursor = editor.get_cursor();
        let code = editor.code_mut();
        code.tx();
        code.set_state_before(cursor, None);

        let pair = format!("{}{}", c, closing);
        code.insert(cursor, &pair);

        let new_cursor = cursor + 1;
        code.set_state_after(new_cursor, None);
        code.commit();

        editor.set_cursor(new_cursor);
        editor.reset_highlight_cache();
        return true;
    }

    // Case 2: Closing character — check if already ahead
    if is_closing(c)
        && let Some(next) = char_at_cursor(editor)
            && next == c {
                // Skip — just move cursor forward
                let cursor = editor.get_cursor();
                editor.set_cursor(cursor + 1);
                return true;
            }

    false
}
