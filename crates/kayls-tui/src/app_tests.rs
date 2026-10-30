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
