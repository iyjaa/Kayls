# Contributing to Kayls

Thanks for your interest in contributing to Kayls! This document explains how to contribute.

---

## Code of Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).

By participating, you are expected to uphold this code. Please report unacceptable behavior to izramulyaputra@gmail.com.

**In short:**

- Be respectful
- Be welcoming
- Be patient
- Accept constructive criticism
- Focus on what's best for the community

---

## Before You Contribute

Please read these documents first:

- [README.md](README.md) — Project overview
- [docs/prd.md](docs/prd.md) — Product requirements
- [docs/versioning.md](docs/versioning.md) — Versioning scheme
- [docs/roadmap.md](docs/roadmap.md) — Feature progress

**Important**: Kayls uses a custom versioning scheme. In `Tsc-v` versions, **no new features** are allowed — only bug fixes. New features start after `Tsc-s`. See [docs/versioning.md](docs/versioning.md) for details.

---

## How to Contribute

Kayls follows a **discuss-first** workflow. Before writing code:

### 1. Discuss (Open an Issue)

Open a GitHub issue describing:

- What you want to change (feature, bug fix, docs)
- Why it's needed
- How you plan to implement it

Wait for maintainer feedback before proceeding.

### 2. Get Approval

The maintainer will review your proposal and either:

- Approve it (you can start coding)
- Suggest changes
- Decline it (with reasoning)

**Do not start coding before approval.** PRs without prior discussion may be closed.

### 3. Fork & Branch

Fork the repository, then create a branch:

    # For new features
    git checkout -b feature/your-feature-name

    # For bug fixes
    git checkout -b fix/your-bug-name

    # For documentation
    git checkout -b docs/your-docs-change

### 4. Code

Follow these guidelines:

- **Rust style** — use `cargo fmt` before committing
- **No warnings** — `cargo build` should not produce warnings
- **Comments** — explain "why", not "what"
- **Small commits** — one logical change per commit

### 5. Test

Before submitting a PR, ensure:

    cargo build --workspace

This must pass without errors. Tests are optional (there are few tests currently), but recommended for new logic.

### 6. Commit

Use [Conventional Commits](https://www.conventionalcommits.org/):

    feat: add new feature
    fix: fix bug in X
    docs: update documentation
    chore: routine task
    refactor: code change without behavior change
    test: add or fix tests

**Examples:**

    feat(tui): add VS Code keybinding mode
    fix(core): handle empty file tree
    docs: update roadmap for Tsc-s1.0.0

### 7. Submit PR

Push your branch and open a Pull Request. In the PR description:

- Reference the issue (e.g., "Closes #12")
- Describe what you changed and why
- Include screenshots if UI changed

### 8. Review

The maintainer will review your PR. Be ready to:

- Answer questions
- Make changes if requested
- Rebase if needed

Once approved, your PR will be merged.

---

## Branch Naming

| Type | Prefix | Example |
|------|--------|---------|
| Feature | `feature/` | `feature/vscode-keybindings` |
| Bug fix | `fix/` | `fix/file-tree-crash` |
| Documentation | `docs/` | `docs/contributing-guide` |
| Refactor | `refactor/` | `refactor/app-structure` |

---

## Commit Message Format

Follow Conventional Commits:

    <type>(<scope>): <description>

    [optional body]

    [optional footer]

**Types:**

- `feat` — new feature
- `fix` — bug fix
- `docs` — documentation
- `chore` — routine tasks
- `refactor` — code change without behavior change
- `test` — tests
- `style` — formatting

**Scopes** (optional):

- `core` — kayls-core
- `tui` — kayls-tui
- `terminal` — kayls-terminal
- `cli` — kayls binary

**Examples:**

    feat(tui): add command palette
    fix(core): file tree root not stable
    docs: add CONTRIBUTING.md

---

## Testing

**Required before PR:**

    cargo build --workspace

**Optional (recommended for new logic):**

    cargo test --workspace

---

## Areas That Need Help

Current priorities (see [docs/roadmap.md](docs/roadmap.md) for details):

### High Priority

- **VS Code keybinding mode** (F-13) — `Ctrl+S`, `Ctrl+O`, etc.
- **Config file** (F-14) — `config.toml` for user settings
- **Multiple file tabs** (F-10) — open multiple files at once
- **Search in file** (F-11) — `Ctrl+F` to find text

### Medium Priority

- **Command palette** (F-12) — fuzzy search for commands
- **Theme selection** — multiple color themes
- **Keybinding customization** — user-defined keybindings

### Documentation

- Improve existing docs
- Add examples
- Translate to other languages (future)

### Bug Fixes

- Check [GitHub Issues](https://github.com/iyjaa/Kayls/issues)
- Look for `good first issue` label

---

## Contact

- **GitHub Issues** — [github.com/iyjaa/Kayls/issues](https://github.com/iyjaa/Kayls/issues)
- **Email** — izramulyaputra@gmail.com

For security issues, please email directly instead of opening a public issue.

---

## License

By contributing to Kayls, you agree that your contributions will be licensed under the [MIT License](LICENSE).

---

## Thank You

Every contribution matters — code, docs, bug reports, or feedback. Thank you for helping make Kayls better.