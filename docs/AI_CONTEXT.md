# Kayls: PRD dan Konteks Proyek untuk AI

Snapshot: commit 1614829 (setelah Kayls-v1.0.10), ditinjau 31 Okt 2026. Dibuat dari pembacaan kode dan docs, belum dibuild atau dijalankan. Nomor baris bisa bergeser, jadi acuan utamanya nama fungsi.

**Cara pakai:** tempel dokumen ini di awal sesi AI, lalu tempel hanya file yang relevan dengan tugasnya (peta file ada di bagian 11).

## 1. Ringkasan produk (PRD)

**Apa itu Kayls.** Terminal Studio Code: IDE berbasis terminal, ditulis dengan Rust dan ratatui. Alternatif ringan untuk Vim, Nano, dan Helix dengan kurva belajar rendah. Lisensi MIT. Repo: github.com/iyjaa/Kayls. Maintainer: iyjaa (izra mulya putra G.).

**Target pengguna.** Developer yang bekerja lewat SSH atau server, pengguna Android/Termux, dan siapa pun yang ingin editor terminal yang langsung bisa dipakai tanpa konfigurasi.

**Masalah dan nilai jual.** Vim menuntut belajar mode dan keybinding, Nano minim fitur, Helix dan Neovim berat dikonfigurasi. Kayls menawarkan satu binary kecil dengan syntax highlighting siap pakai, file explorer, pencarian, dan dua gaya keybinding: Easy Mode ala VS Code dan Command Mode terinspirasi Vim.

**Non-goals saat ini.** Bukan klon Vim 1:1 (Esc membuka menu perintah satu tombol, bukan Normal Mode Vim). Belum ada LSP, plugin, split pane, atau tabs (semuanya di roadmap). Belum ada rilis untuk Windows (installer mendukung Linux, macOS, Android/Termux).

**Target kinerja (dari PRD, belum diukur otomatis di CI).** Startup di bawah 100 ms dan ukuran binary di bawah 5 MB. Profil release: opt-level z, LTO, codegen-units 1, strip, panic abort.

**Status fitur (v1.0.10)**

| Fitur | Status |
|---|---|
| Editing: undo/redo, auto-pair, auto-indent, duplicate/delete line, select all, toggle comment, indent/unindent | Ada |
| Syntax highlighting tree-sitter (rust, python, js/jsx, ts/tsx, go, java, c, cpp, c#, html, css, json, toml, yaml, shell, markdown) | Ada |
| File Explorer: buat file/folder, hapus dengan konfirmasi, warna gradient menurut kedalaman folder | Ada |
| File picker fuzzy (Ctrl+P), pencarian dalam file (Ctrl+F), Read Mode (Ctrl+R) | Ada |
| 4 tema: Vesper (default), Gruvbox, Dracula, Nord | Ada |
| 2 mode keybinding: Easy dan Command Mode (Vim-inspired) | Ada |
| Mouse (klik, drag, scroll) | Ada, opsional, default mati |
| Konfigurasi TOML di ~/.config/kayls/config.toml | Ada |
| Installer, repo APT, build Termux (aarch64), macOS, Linux x86_64/arm64 | Ada |
| Rename file/folder, replace teks, multi-tabs, split pane, multi-cursor, LSP, True Vim Mode | Belum |

**Roadmap (dari docs/ROADMAP.md)**
- v1.1.x: Multi-Tabs, layout responsif untuk layar sempit, True Vim Mode, multi-cursor (Alt+D), tutorial Termux.
- v1.2.x dan seterusnya: dukungan LSP, split panes.

## 2. Arsitektur

Workspace Cargo dengan 4 crate, sekitar 8.000 baris Rust (editor sekitar 5.000, tui sekitar 2.100, core sekitar 750).

| Crate | Peran | File penting |
|---|---|---|
| kayls | Binary: parsing CLI (clap), membuka file awal, memanggil kayls_tui::run | src/main.rs |
| kayls-tui | Aplikasi TUI: state, event loop, handler tombol, render | app.rs (1.045 baris), ui.rs (694), picker.rs, search.rs, lib.rs |
| kayls-core | Logika non-UI: Document, FileTree, Config, find_matches | filetree.rs, config.rs, document.rs, lib.rs |
| kayls-editor | Widget editor (fork ratatui-code-editor): rope, tree-sitter, history, aksi, render | editor.rs (1.097), code.rs (1.184), actions.rs (893), view.rs, render.rs, diff.rs, editor_crossterm.rs |

**Aliran dependensi:** kayls (bin) memanggil kayls-tui, yang memakai kayls-core dan kayls-editor.

**Stack:** Rust edition 2024, ratatui 0.30, crossterm 0.29, ropey 1.6, tree-sitter 0.26 beserta grammar di crates/kayls-editor/langs (juga ada kotlin, lua, zig, text, markdown-inline), clap 4, serde dan toml 0.8, anyhow, criterion untuk benchmark.

## 3. Fungsi dan tipe penting

### kayls-core

| Simbol | Fungsi |
|---|---|
| Document::open(path), Document::empty() | Membaca file lewat read_to_string. Field path dan rope. PENTING: rope hanya snapshot saat file dibuka, jangan dibaca setelah Editor dibuat |
| Document::save, insert_char, delete_char, dirty | Logika lama yang tidak dipakai TUI (kode mati). Penyimpanan nyata ada di App.handle_command tombol w |
| FileTree::from_path(root) | Membangun pohon file. Metode: rebuild_flat, move_up/move_down, toggle_expand_selected, create_file, create_folder, delete_selected, selected_entry, iter_visible, find_entry, contains_path |
| Config::load / save / config_path | File ~/.config/kayls/config.toml (hanya dari env HOME). EditorConfig: theme, show_explorer, keybinding (vim atau easy), mouse_capture |
| find_matches(code, query) | Pencarian case-insensitive, hasil berupa indeks KARAKTER (bukan byte), sliding window tanpa alokasi Vec<char> seluruh dokumen |

### kayls-editor

| Simbol | Fungsi |
|---|---|
| Editor::from_rope(lang, rope, theme), Editor::new(lang, text, theme) | Membuat editor, mengembalikan Result |
| editor.apply(Action) | Jalur tunggal perubahan: InsertText, InsertNewline, Delete, DeleteLine, Duplicate, Cut, Copy, Paste, Undo, Redo, Indent, UnIndent, ToggleComment, SelectAll, MoveLeft/Right/Up/Down |
| editor.input(key, area) | Memetakan KeyEvent ke Action |
| is_dirty(), mark_saved() | Dirty = revision berbeda dari revision saat disimpan |
| get_content(), set_cursor, scroll_to_cursor, focus, set_theme, show_line_numbers | API yang dipakai TUI |
| cursor_from_mouse, handle_mouse_down, handle_mouse_drag | Dukungan mouse |
| Code (code.rs) | Pembungkus Rope, tree-sitter, dan history. insert/remove menaikkan revision. tx()/commit() mengelompokkan edit. undo()/redo() |

### kayls-tui

| Simbol | Fungsi |
|---|---|
| run(document) | Memuat config, panic hook, ratatui::init, mouse capture opsional, run_loop |
| run_loop | Hitung layout, app.tick(), terminal.draw, poll event adaptif, dispatch |
| App (app.rs) | State tunggal: document, editor, file_tree, mode, saved_mode, focus, theme, clipboard, search, picker, dll |
| Mode | Normal, Easy, Command, Prompt, PromptNewFile, PromptNewFolder, ConfirmDelete, SelectTheme, SelectKeybinding, Search, FilePicker |
| handle_key | Dispatcher utama per mode |
| load_editor, open_file_from_path, detect_language | Membuat Editor, membuka file, memetakan ekstensi |
| SearchState (search.rs) | query, matches, current_idx. perform_search, next_match, reset |
| PickerState (picker.rs) | update_matches, handle_key, render |

## 4. Alur kerja runtime

**Startup:** main.rs parse `kayls [file]`, Document::open (keluar kode 1 bila gagal), kayls_tui::run, Config::load, App::new (membaca theme, show_explorer, keybinding untuk menentukan Mode::Easy atau Normal), load_file_tree dari direktori kerja, load_editor bila ada dokumen.

**Event loop dan handle_key:**
- Key event -> abaikan jika bukan Press
- Alt+Q: quit (ditolak bila editor.is_dirty())
- dispatch menurut self.mode
- Esc di Normal/Easy -> Mode::Command (saved_mode mengingat mode sebelumnya)
- Setiap perintah selesai dengan self.mode = self.saved_mode

**Editing:** handle_editor_keys menangani shortcut tingkat aplikasi lebih dulu (Ctrl+F, Ctrl+R, Ctrl+P, Alt+C/Ctrl+C, Ctrl+X, Ctrl+V yang disinkronkan ke App.clipboard), sisanya diteruskan ke editor.input, lalu Action, lalu Code.insert/remove, lalu revision naik.

**Simpan:** Esc w atau Ctrl+S -> handle_command tombol w menulis editor.get_content() ke path dengan fs::write, lalu editor.mark_saved().

**Penjaga dirty** (memakai editor.is_dirty(), pesan Unsaved changes): quit, Ctrl+O, Ctrl+N, Esc o, Esc n, pilihan file picker, Enter di Explorer. Tidak ada autosave.

**Buka file:** Enter di Explorer -> open_file_from_path, cek dirty, Document::open, load_editor, fokus ke editor.

**Pencarian:** Ctrl+F masuk Mode::Search. Tiap karakter memanggil SearchState::perform_search(editor.get_content()). Enter memanggil next_match lalu set_cursor dan scroll_to_cursor. Esc keluar dan reset.

**Tema:** Esc t membuka SelectTheme, lalu editor.set_theme pada editor yang ada (tidak dibangun ulang), lalu Config disimpan.

**Keybinding:** Esc m membuka SelectKeybinding, pilihan disimpan ke config.

**Mouse:** hanya aktif bila mouse_capture = true di config (default false) dan hanya diproses di mode Normal atau Easy.

**Layout:** lib.rs menghitung editor_area (x = 21 bila Explorer tampil, 1 bila tersembunyi, y = 2, lebar = W - 22 atau W - 2, tinggi = H - 4). Angka ini HARUS sinkron dengan ui.rs.

## 5. Keybinding ringkas

| Konteks | Tombol |
|---|---|
| Global | Esc (menu perintah), Alt+Q (quit), Ctrl+F (cari), Ctrl+R (Read Mode), Ctrl+P (file picker) |
| Easy Mode | Ctrl+S simpan, Ctrl+O buka, Ctrl+N file baru, Ctrl+Q keluar |
| Command Mode (setelah Esc) | o buka, n file baru, v folder baru, d hapus, t tema, w simpan, e fokus Explorer, q tutup Explorer, m ganti mode keybinding, Esc batal |
| Edit | Ctrl+Z undo, Ctrl+Y redo, Ctrl+D duplikat baris, Ctrl+K hapus baris, Ctrl+A pilih semua, Alt+C/Ctrl+C salin, Ctrl+X potong, Ctrl+V tempel |
| Auto-pair | ( { [ < tanda kutip ganda dan tunggal menyisipkan penutupnya |

Di Termux, tempel dari clipboard sistem memakai tombol PASTE (bracketed paste, ditangani App::paste).

## 6. Konfigurasi

File `~/.config/kayls/config.toml` (dibuat otomatis saat menyimpan), bagian `[editor]`:
- `theme` (vesper, gruvbox, dracula, nord)
- `show_explorer` (true atau false)
- `keybinding` (vim atau easy)
- `mouse_capture` (default false)

Hanya env HOME yang dibaca. File yang gagal diparse diam-diam diganti nilai default.

## 7. Pengembangan dan rilis

**Branch:** main dan cabang fitur feat/* atau docs/* lewat PR. CI jalan untuk push dan PR ke main, feat/, docs/: cargo fmt --check, cargo clippy --all-targets -D warnings, cargo test --all.

**Versioning:** tag `Kayls-v<MAJOR.MINOR.PATCH>` untuk stabil dan `Kayls-s<versi>-beta.N` untuk snapshot.

**Versi ada di tiga tempat yang harus sinkron:** `[workspace.package]` di Cargo.toml, file VERSION, dan Cargo.lock.

**scripts/release.sh:** input versi, update Cargo.toml dan VERSION, cargo update -w, buat branch release/vX, commit, push, buka PR. Tag dibuat manual di main setelah PR di-merge.

**scripts/retag.sh:** hapus dan buat ulang tag di main yang sinkron dengan origin. Ini menimpa rilis yang sama, hindari bila bisa.

**Push tag memicu release.yml:** job check, build Linux x86_64 dan arm64, build macOS x86_64 dan arm64, build-deb-and-push (paket deb dan repo APT), lalu job release yang membuat checksum per file (<arsip>.sha256) dan GitHub Release. release-android.yml membangun target aarch64-linux-android (Termux) secara paralel dan ikut menunggu check.

**install.sh:** mendeteksi OS dan arsitektur (Linux x86_64 dan arm64, Termux aarch64, macOS x86_64 dan arm64), mengunduh arsip dan file .sha256, memverifikasi dengan sha256sum atau shasum -a 256. Mode ketat lewat KAYLS_STRICT_CHECKSUM=1.

**ISSUES.md** dibuat otomatis oleh workflow update-issues dari GitHub Issues.

## 8. Aturan emas untuk AI yang mengubah kode

1. Editor adalah sumber kebenaran teks. Jangan membaca Document.rope setelah load_editor, dan jangan membangun ulang editor dari Document kecuali saat membuka file.
2. Semua perubahan teks lewat Action atau Code.insert/remove agar revision, history, dan highlight tetap konsisten. Jangan memodifikasi content langsung.
3. Status belum tersimpan hanya dari editor.is_dirty(). Jangan membuat flag dirty baru.
4. Setiap handler modal harus kembali ke self.saved_mode, bukan Mode::Normal.
5. Semua posisi di editor dan pencarian berupa indeks KARAKTER. Jangan memotong string dengan indeks byte.
6. Perubahan ukuran Explorer atau border harus diubah bersamaan di lib.rs (editor_area) dan ui.rs, atau dijadikan satu fungsi layout.
7. Mouse capture default mati. Jangan mengaktifkannya tanpa lewat config.
8. Rilis memakai panic = abort, jadi hindari unwrap dan indexing yang bisa panic di jalur input pengguna.
9. CI memakai clippy -D warnings. Jalankan cargo fmt, cargo clippy --all-targets, dan cargo test --all sebelum menyerahkan patch.
10. Teks UI dan komentar campuran Indonesia dan Inggris. Ikuti gaya file yang disentuh.

## 9. Review: kekuatan dan hal yang perlu ditingkatkan

**Kekuatan.** Pemisahan crate jelas dan inti editor terpisah dari UI. Dirty tracking berbasis revision (termasuk undo dan redo). Pencarian aman Unicode dan CRLF dengan test. Panic hook memulihkan terminal. Mouse opsional. Checksum per file. CI lengkap dan rilis menunggu check.

**P0: risiko kehilangan data atau penghalang fitur berikutnya**
1. Tidak ada crash recovery. Karena panic = abort, panic berarti buffer hilang.
2. Penyimpanan tidak atomik. fs::write langsung ke file tujuan. Tulis ke file sementara lalu rename.
3. State tunggal Option<Editor> ditambah Document dengan rope snapshot. Ganti dengan Vec<Tab> (bagian 10) dan hapus kode mati di Document.
4. Cek dirty tersebar di sekitar 7 tempat. Buat satu helper can_discard().
5. Layout dihitung di dua tempat (lib.rs dan ui.rs) dengan angka magic (H - 4, y = 2). Jadikan satu fungsi layout.

**P1: kualitas dan kelengkapan**
6. Belum ada rename file/folder.
7. Explorer tidak merespons mouse atau scroll.
8. Clipboard hanya internal ditambah bracketed paste. Tambahkan OSC 52.
9. Pemetaan bahasa ganda dan berbeda: detect_language di app.rs dan get_lang di editor utils.rs.
10. Fitur editor yang tidak dipakai TUI (diff view, code folding, word highlight, marks).
11. Config: hanya HOME (tidak ada XDG_CONFIG_HOME atau USERPROFILE).
12. Pencarian mengalokasikan String seluruh dokumen dan scan penuh di setiap ketikan.
13. File non-UTF-8 gagal dibuka tanpa fallback.
14. Test tipis. Belum ada untuk FileTree, Config, transisi Mode, dan handle_key.
15. Rilis: installer lanjut tanpa verifikasi bila checksum tidak ada.
16. Target startup di bawah 100 ms dan binary di bawah 5 MB belum diukur.

**P2: roadmap**
Multi-Tabs, layout responsif, True Vim Mode, multi-cursor, tutorial Termux, split panes, LSP.

## 10. Rancangan Multi-Tabs (usulan)

- Struktur: `struct Tab { path: PathBuf, editor: Editor }` dan `App { tabs: Vec<Tab>, active: usize }`. Hapus field document dan editor tunggal. Sediakan `active_editor()` dan `can_discard(tab_idx)`.
- Membuka file yang sudah terbuka berpindah ke tab itu. Menutup tab yang dirty meminta konfirmasi. Quit memeriksa semua tab.
- UI: tab bar di baris atas (nama file dan titik bila dirty) dengan scroll horizontal. Hitung layout dari satu fungsi yang dipakai ui.rs dan run_loop.
- Keybinding usulan: Alt+Kiri dan Alt+Kanang pindah tab, Ctrl+W tutup di Easy Mode.
- Rilis lewat snapshot Kayls-s1.1.0-beta.1. Test: dirty per tab, menutup tab terakhir, membuka file yang sama dua kali.

## 11. Cara memakai dokumen ini dengan AI

Tempel dokumen ini, lalu hanya file yang relevan:

| Tugas | File yang ditempel |
|---|---|
| Multi-Tabs, state aplikasi, mode | app.rs (struct App, handle_key, handler terkait, load_editor), lib.rs, bagian layout di ui.rs |
| Keybinding | editor_crossterm.rs, handle_editor_keys, handle_easy, handle_command di app.rs, KEYBINDS.md |
| Pencarian | search.rs, find_matches di kayls-core/lib.rs, handle_search di app.rs |
| Internal editor (undo, highlight, aksi) | code.rs, actions.rs, history.rs |
| Explorer | filetree.rs, handle_explorer_keys di app.rs, bagian explorer di ui.rs |
| Tema dan warna | theme.rs, ui.rs |
| CI, rilis, installer | .github/workflows/.yml, scripts/.sh, install.sh, docs/VERSIONING.md |

**Contoh prompt:** Konteks proyek ada di dokumen di atas. Tugas: implementasikan Multi-Tabs sesuai bagian 10. Berikut app.rs, lib.rs, dan ui.rs. Patuhi aturan emas di bagian 8 dan berikan patch dalam bentuk diff yang lolos cargo fmt dan clippy -D warnings.

Perbarui dokumen ini tiap rilis (versi, status fitur, daftar masalah) supaya konteks AI tidak basi.
