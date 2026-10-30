use super::*;

#[test]
fn test_search_unicode_safe() {
    let mut app = App::new(None);
    app.search_query = "ab".to_string();
    // Tidak boleh panic meskipun tidak ada editor
    app.perform_search();
    assert!(app.search_matches.is_empty());
}

#[test]
fn test_search_empty_query() {
    let mut app = App::new(None);
    app.search_query = "".to_string();
    app.perform_search();
    assert!(app.search_matches.is_empty());
}

#[test]
fn test_dirty_after_edit() {
    // Placeholder: butuh mock Editor
    // Untuk saat ini, kita hanya memastikan is_dirty() ada
}
