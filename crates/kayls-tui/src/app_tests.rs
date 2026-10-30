use super::*;

#[test]
fn test_find_matches_unicode_safe() {
    // "aé" adalah 2 karakter: 'a' dan 'é'
    // Query "ab" tidak boleh panic dan harus kosong
    assert!(find_matches("aé", "ab").is_empty());

    // Query "a" harus menemukan match di indeks 0
    assert_eq!(find_matches("aé", "a"), vec![0]);

    // Query "é" harus menemukan match di indeks 1
    assert_eq!(find_matches("aé", "é"), vec![1]);
}

#[test]
fn test_find_matches_empty_query() {
    assert!(find_matches("hello", "").is_empty());
}

#[test]
fn test_find_matches_normal() {
    assert_eq!(find_matches("hello hello", "hello"), vec![0, 6]);
}

#[test]
fn test_dirty_after_edit() {
    use kayls_editor::actions::InsertText;
    use kayls_editor::editor::Editor;

    let mut editor = Editor::new("rust", "test", vec![]).unwrap();
    assert!(!editor.is_dirty());

    editor.apply(InsertText {
        text: "hello".to_string(),
    });
    assert!(editor.is_dirty());

    editor.mark_saved();
    assert!(!editor.is_dirty());

    editor.apply(kayls_editor::actions::Undo {});
    assert!(editor.is_dirty());
}
