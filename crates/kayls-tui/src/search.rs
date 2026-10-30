use kayls_core::find_matches;

pub struct SearchState {
    pub query: String,
    pub matches: Vec<usize>,
    pub current_idx: usize,
    pub scan_offset: usize,
    pub scan_complete: bool,
    pub entered: bool,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            matches: Vec::new(),
            current_idx: 0,
            scan_offset: 0,
            scan_complete: true,
            entered: false,
        }
    }

    pub fn reset(&mut self) {
        self.matches.clear();
        self.current_idx = 0;
        self.scan_offset = 0;
        self.scan_complete = true;
        self.entered = false;
    }

    /// Cari di 10.000 baris pertama (instan)
    pub fn perform_search(&mut self, code: &str) {
        self.matches.clear();
        self.current_idx = 0;
        self.scan_offset = 0;
        self.scan_complete = true;
        self.entered = false;

        if self.query.is_empty() {
            return;
        }

        let lines: Vec<&str> = code.lines().collect();
        let end_line = lines.len().min(10_000);
        let partial = lines[..end_line].join("\n");

        self.matches = find_matches(&partial, &self.query);
        self.scan_offset = end_line;
        self.scan_complete = end_line >= lines.len();
    }

    /// Lanjutkan pencarian ke 10.000 baris berikutnya
    pub fn continue_search(&mut self, code: &str) {
        if self.scan_complete || self.query.is_empty() {
            return;
        }

        let lines: Vec<&str> = code.lines().collect();
        let start = self.scan_offset;
        let end = (start + 10_000).min(lines.len());

        if start >= end {
            self.scan_complete = true;
            return;
        }

        let partial = lines[start..end].join("\n");
        let new_matches = find_matches(&partial, &self.query);
        let char_offset: usize = lines[..start].iter().map(|l| l.chars().count() + 1).sum();
        self.matches
            .extend(new_matches.iter().map(|m| m + char_offset));

        self.scan_offset = end;
        self.scan_complete = end >= lines.len();
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
