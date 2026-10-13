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

### Examples are illustrations only
- Examples in `versioning.md` (X=1 Terminal, X=2 LSP, X=3 AI Agent) are illustrations
- Actual feature progress lives in `roadmap.md`
- This is noted in `README.md`

### Next TODO
- [ ] Dogfood 3 days, then promote `Tsc-v0.1.0` to `Tsc-s0.1.0`
- [ ] Update `Cargo.toml` to read from `VERSION` (build.rs)
- [ ] Start `Tsc-v0.2.0`: editor basic layout

---

## 2026-10-13 — Initial Setup

### Done
- Set up Rust workspace with 5 crates
- Root `Cargo.toml` with `[workspace]`
- Crates: tscode-core, tscode-syntax, tscode-terminal, tscode-tui, tscode
- Git init & push to GitHub (https://github.com/iyjaa/Tscode)
- `cargo build --workspace` succeeded
- `cargo run -p tscode` → `Tscode v0.1.0`

### Decisions
- **Workspace from the start** — to scale up to v1.0
- **Version 0.1.0** in root `[workspace.package]`, all crates use `version.workspace = true`
- **Non-modal** — VS Code keybindings, not Vim
- **Rust first** for syntax highlighting

### Notes
- `cargo new` creates `.git` & `.gitignore` inside each crate → must be removed
- Use `--vcs none` for new crates going forward
- First build is slow (~6 min), subsequent builds are fast

### Next TODO
- [ ] Clean up `Cargo.toml` (fix email, add description)
- [ ] `main.rs` with clap (`--version`, `--help`)
- [ ] v0.2.0: TUI layout (tab bar, file tree, editor, status bar)
- [ ] v0.2.0: Open file & display its contents

---

## Template for New Entries

```markdown
## YYYY-MM-DD — Title

### Done
- 

### Decisions
- 

### Issues & Solutions
- 

### Next TODO
- [ ] 