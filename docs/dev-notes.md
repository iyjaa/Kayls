# Dev Notes

Personal notes during Tscode development. Not public documentation.

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
- `Tsc-v0.2.3` — nomor baris di editor (in progress)

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

### Known Issues
- **Explorer masih hardcoded** — cuma teks statis, belum baca folder asli
- **Toggle explorer tidak bekerja** di terminal Acode — `Ctrl+B` diteruskan ke Acode
- **`Cargo.toml` versi tidak sinkron** — root masih `0.2.0`, sementara `VERSION` sudah `0.2.3`

### Files Updated (v0.2.x)
- `crates/tscode-core/src/document.rs` — `Document` struct
- `crates/tscode-core/src/cursor.rs` — `Cursor` struct
- `crates/tscode-core/src/lib.rs` — ekspos `Document`, `Cursor`
- `crates/tscode-tui/src/app.rs` — `App` state, handle key
- `crates/tscode-tui/src/ui.rs` — render editor, nomor baris
- `crates/tscode-tui/src/lib.rs` — `run()` terima `Document`
- `crates/tscode/src/main.rs` — parsing argumen dengan `clap`
- `crates/tscode/build.rs` — baca `VERSION`

### Next TODO
- [ ] Selesaikan `Tsc-v0.2.3` — nomor baris rapi
- [ ] Promosi `Tsc-s0.2.3`
- [ ] `Tsc-v0.2.4` — insert karakter (editor bisa ngetik)
- [ ] `Tsc-v0.2.5` — delete (Backspace)
- [ ] `Tsc-v0.2.6` — save (`Ctrl+S`)
- [ ] `Tsc-v0.3.0` — file tree fungsional

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