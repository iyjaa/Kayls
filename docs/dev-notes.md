# Dev Notes

Personal notes during Tscode development. Not public documentation.

---

## 2026-10-14 — File Tree Renders Actual Directory

### Done
- Tsc-v0.3.0: `FileTree` struct untuk baca folder
- Tsc-v0.3.1: `App` render file tree di explorer
- Promosi ke `Tsc-s0.3.1`

### Decisions
- **FileTree di `tscode-core`** — logika baca folder, bukan rendering
- **`fs::canonicalize()`** di `Document::open` — resolve path relatif jadi absolut
- **Fallback ke current directory** — kalau parent kosong
- **Explorer 20 kolom** — cukup untuk nama file, editor tetap lega
- **Icon `+` untuk folder** — `▸` tidak render di semua terminal
- **Urutan: folder dulu, file, alfabetis** — standar file manager

### Issues & Solutions
- **Path relatif `Cargo.toml`** -> `doc.path.parent()` kosong -> `FileTree error: Not a directory`
  - Solved: `fs::canonicalize()` di `Document::open`
  - Fallback: `std::env::current_dir()`

### Files Created
- `crates/tscode-core/src/filetree.rs` — `FileTree`, `Entry`

### Files Updated
- `crates/tscode-core/src/lib.rs` — ekspos `FileTree`, `Entry`
- `crates/tscode-core/src/document.rs` — `canonicalize()`
- `crates/tscode-tui/src/app.rs` — `file_tree` field, `load_file_tree()`
- `crates/tscode-tui/src/ui.rs` — `render_file_tree()`
- `crates/tscode-tui/src/lib.rs` — `editor_width` (explorer 20 kolom)

### Next TODO
- [ ] `Tsc-v0.3.2` — navigasi arrow di file tree
- [ ] `Tsc-v0.3.3` — buka file dengan Enter
- [ ] `Tsc-v0.3.4` — expand/collapse folder
- [ ] `Tsc-v0.3.5` — toggle explorer (`Esc` -> `e`)

### Notes
- **`show_explorer: true`** — sementara biar kelihatan. Nanti bisa di-toggle.
- **Focus state belum ada** — arrow keys masih untuk editor. Nanti tambah `Focus::Editor` / `Focus::Explorer`.

---

## 2026-10-13 — Versioning Scheme Decided

### Decision
- Custom versioning: `Tsc-vX.Y.Z` (unstable) and `Tsc-sX.Y.Z` (stable)
- X: major feature (planned in roadmap)
- Y: sub-feature (discovered during development, based on minimum for feature to work)
- Z: bug fix count
- Stability: 3 days dogfooding + tests pass + no known bugs
- Bug in stable version: prefix drops to `v`, Z increments
- `Cargo.toml`: numeric only (`0.1.0`), full version in `VERSION` file
- Git: 2 tags per version (`Tsc-vX.Y.Z` and `Tsc-sX.Y.Z`)
- Golden rule: in `Tsc-v`, no new features. Only bug fixes.

### Rationale
- Enforces discipline: no new features while in `Tsc-v`
- Quality first: bugs cleared before moving on
- Clear for future contributors: they know the rule
- Solo-dev friendly: forces focus

### Files Created
- `VERSION` (root) — contains `Tsc-v0.1.0`
- `docs/versioning.md` — full versioning documentation

### Files Updated
- `README.md` — status uses `Tsc-v0.1.0`, note about illustrations
- `docs/prd.md` — added section 11 (Versioning)
- `docs/roadmap.md` — rewritten with `Tsc-v` / `Tsc-s` scheme

### Next TODO
- [ ] Dogfood 3 days, then promote `Tsc-v0.1.0` to `Tsc-s0.1.0`
- [ ] Start `Tsc-v0.2.0`: editor basic layout

---

## 2026-10-14 — Tsc-v0.2.x Progress

### Done
- `Tsc-s0.1.0` — promote versioning setup
- `Tsc-v0.2.0` — layout TUI statis (tab bar, explorer, editor, status bar)
- `Tsc-s0.2.0` — promote layout
- `Tsc-v0.2.1` — buka file via argumen (`tscode file.rs`)
- `Tsc-s0.2.1` — promote buka file
- `Tsc-v0.2.2` — cursor movement (arrow keys, highlight, status posisi)
- `Tsc-s0.2.2` — promote cursor
- `Tsc-v0.2.3` — nomor baris di editor
- `Tsc-s0.2.3` — promote nomor baris
- `Tsc-v0.2.4` — insert, delete, save, mode, hint
- `Tsc-s0.2.4` — promote editor dasar

### Decisions
- **Workspace dari awal** — scalable sampai v1.0
- **Non-modal** — keybinding VS Code, bukan Vim
- **Rust first** untuk syntax highlighting
- **`Document` di `tscode-core`** — logika file di core, rendering di tui
- **`Cursor` terpisah** — `tscode-core/src/cursor.rs`
- **File tree 12 kolom** (Length) — hemat ruang di layar HP
- **Nomor baris 3 kolom** — cukup untuk 2 digit
- **Warna `Gray`** untuk nomor baris — terang tapi tidak mengganggu
- **Toggle explorer sementara di-skip** — karena `Ctrl+B` bentrok dengan Acode
- **Vim-style command mode** — `Esc` -> `o` (open), `w` (write)
- **Quit via `Alt+Q`** — karena `Ctrl+Q` bentrok Acode
- **Hint 2 detik** setelah save — pakai timer `Instant`

### Known Issues
- **Explorer masih hardcoded** — cuma teks statis (solved di `Tsc-v0.3.1`)
- **Toggle explorer tidak bekerja** di terminal Acode — `Ctrl+B` diteruskan ke Acode
- **`Cargo.toml` versi tidak sinkron** — root masih `0.2.0`, sementara `VERSION` sudah `0.2.3`

### Files Updated (v0.2.x)
- `crates/tscode-core/src/document.rs` — `Document` struct
- `crates/tscode-core/src/cursor.rs` — `Cursor` struct
- `crates/tscode-core/src/lib.rs` — ekspos `Document`, `Cursor`
- `crates/tscode-tui/src/app.rs` — `App` state, handle key, `Mode` enum
- `crates/tscode-tui/src/ui.rs` — render editor, nomor baris, status bar
- `crates/tscode-tui/src/lib.rs` — `run()` terima `Document`
- `crates/tscode/src/main.rs` — parsing argumen dengan `clap`
- `crates/tscode/build.rs` — baca `VERSION`

### Next TODO
- [ ] Selesaikan `Tsc-v0.3.1` — render file tree
- [ ] `Tsc-v0.3.2` — navigasi arrow di file tree
- [ ] `Tsc-v0.3.3` — buka file dengan Enter
- [ ] `Tsc-v0.3.4` — expand/collapse folder
- [ ] `Tsc-v0.3.5` — toggle explorer

### Notes
- **Git detached HEAD** pernah terjadi saat rebase — sudah diperbaiki dengan `--force push`
- **Tag `1`** pernah terbuat tidak sengaja — sudah dihapus
- **Build pertama lama (~6 menit)**, selanjutnya cepat
- **`cargo new` bikin `.git` & `.gitignore`** di tiap crate — harus dihapus, pakai `--vcs none` ke depannya

---

## Template for New Entries

## YYYY-MM-DD — Title

### Done
- 

### Decisions
- 

### Issues & Solutions
- 

### Next TODO
- [ ] 