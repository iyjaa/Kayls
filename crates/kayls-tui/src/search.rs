use kayls_core::find_matches;

pub struct SearchState {
    pub query: String,
    pub matches: Vec<usize>,
    pub current_idx: usize,
    pub entered: bool,
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            matches: Vec::new(),
            current_idx: 0,
            entered: false,
        }
    }

    pub fn reset(&mut self) {
        self.matches.clear();
        self.current_idx = 0;
        self.entered = false;
    }

    pub fn perform_search(&mut self, code: &str) {
        self.matches.clear();
        self.current_idx = 0;
        self.entered = false;

        if self.query.is_empty() {
            return;
        }

        self.matches = find_matches(code, &self.query);
    }

    pub fn next_match(&mut self) -> Option<usize> {
        if self.matches.is_empty() {
            return None;
        }
        if self.entered {
            self.current_idx = (self.current_idx + 1) % self.matches.len();
        } else {
            self.entered = true;
        }
        Some(self.matches[self.current_idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_unicode() {
        let mut s = SearchState::new();
        s.query = "é".to_string();
        s.perform_search("aé b é");
        assert_eq!(s.matches, vec![1, 5]);
    }

    #[test]
    fn test_search_crlf() {
        let mut s = SearchState::new();
        s.query = "hello".to_string();
        s.perform_search("hello\r\nworld\r\nhello");
        assert_eq!(s.matches, vec![0, 14]);
    }

    #[test]
    fn test_search_large_file() {
        let mut s = SearchState::new();
        s.query = "hello".to_string();
        let code = "hello world\n".repeat(100_000);
        s.perform_search(&code);
        assert_eq!(s.matches.len(), 100_000);
    }
}
