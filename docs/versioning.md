# Tscode Versioning

Tscode uses a custom versioning scheme to enforce discipline and quality.

---

## Format

Tsc-vX.Y.Z   ->  Unstable
Tsc-sX.Y.Z   ->  Stable

| Position | Meaning | How Determined |
|----------|---------|----------------|
| X | Major feature | Planned in roadmap |
| Y | Sub-feature inside X | Discovered during development |
| Z | Bug fix count for X.Y | Incremented on each fix |

---

## Meaning of X, Y, Z

### X — Major Feature

A complex, standalone feature. Examples:

- X=1: Integrated terminal
- X=2: LSP
- X=3: AI agent
- X=4: Plugin system

X is planned ahead in the roadmap.

### Y — Sub-feature

A sub-feature inside a major feature. The number of Y is not known in advance. It is discovered while building the feature, based on the minimum requirements for it to work.

Example for X=3 (AI Agent):

| Y | Sub-feature | Minimum to Work |
|---|-------------|-----------------|
| 1 | AI project understanding | AI can read project structure |
| 2 | AI type provider | Select model (GPT, Claude, Gemini) |
| 3 | AI provider integration | Connect to ChatGPT, Claude, etc. |
| 4 | ... | (discovered later) |

If X=3 turns out to need 4 sub-features, it finishes at Y=4. If it needs 7, it finishes at Y=7.

### Z — Bug Fix Count

How many times a bug has been fixed at X.Y. Starts at 0.

Example:

- Tsc-v1.2.0 — sub-feature 2 done, no bugs yet
- Tsc-v1.2.1 — bug found, fixed (fix #1)
- Tsc-v1.2.2 — another bug fixed (fix #2)
- Tsc-s1.2.2 — stable after 3 days of dogfooding

---

## Transition Rules

1. New major feature          -> Tsc-vX.0.0
2. Sub-feature done           -> Tsc-vX.Y.0
3. Bug found                  -> Tsc-vX.Y.Z
4. Bug fixed                  -> Tsc-vX.Y.Z+1
5. Stable (3 days dogfood)    -> Tsc-sX.Y.Z
6. Next sub-feature           -> Tsc-vX.Y+1.0
7. All sub-features done      -> Tsc-sX.N.Z
8. Next major feature         -> Tsc-vX+1.0.0

---

## Stability Criteria

A Tsc-v version becomes Tsc-s when ALL of these are true:

1. All unit tests pass (`cargo test --workspace`)
2. Dogfooding for 3 days without new bugs
3. No known bugs in the issue tracker
4. You personally use it for real work

---

## Golden Rule

| Phase | Prefix | Allowed | Forbidden |
|-------|--------|---------|-----------|
| Development | Tsc-v | Bug fixes, tests, docs | Adding new features |
| Stable | Tsc-s | Adding new features (goes to Tsc-v) | - |

Tsc-v is a feature freeze. Only bug fixes. New features wait until Tsc-s.

---

## Examples

### Example 1: Terminal (X=1, 5 sub-features)

Tsc-v1.0.0   start X=1, no sub-features yet
Tsc-v1.1.0   sub-feature 1 done (spawn shell)
Tsc-v1.1.1   bug in sub-feature 1 fixed (fix #1)
Tsc-v1.1.2   bug in sub-feature 1 fixed (fix #2)
Tsc-s1.1.2   sub-feature 1 stable
Tsc-v1.2.0   sub-feature 2 done (send input)
Tsc-s1.2.0   sub-feature 2 stable
Tsc-v1.3.0   sub-feature 3 done (display output)
Tsc-s1.3.0   sub-feature 3 stable
Tsc-v1.4.0   sub-feature 4 done (resize)
Tsc-s1.4.0   sub-feature 4 stable
Tsc-v1.5.0   sub-feature 5 done (multi-tab)
Tsc-v1.5.1   bug in sub-feature 5 fixed
Tsc-s1.5.1   sub-feature 5 stable
Tsc-v2.0.0   start X=2 (LSP)

### Example 2: Bug in Stable Version

Tsc-s1.5.1   stable
             (bug discovered)
Tsc-v1.5.2   bug fixed, prefix drops to v
Tsc-s1.5.2   stable again after 3 days

---

## Cargo.toml Workaround

Cargo.toml only accepts numeric X.Y.Z. The full version with prefix lives in a VERSION file at the repo root.

VERSION file:

    Tsc-v0.1.0

Cargo.toml:

    [workspace.package]
    version = "0.1.0"    # numeric only

build.rs reads VERSION and exposes it as an env var:

    fn main() {
        let version = std::fs::read_to_string("VERSION")
            .unwrap_or_else(|_| "Tsc-v0.0.0".to_string());
        println!("cargo:rustc-env=TSC_VERSION={}", version.trim());
    }

main.rs:

    fn main() {
        println!("Tscode {}", env!("TSC_VERSION"));
    }

Output: Tscode Tsc-v0.1.0

---

## Git Tags

Two tags per version:

    # When releasing Tsc-v1.2.0
    git tag Tsc-v1.2.0
    git push origin Tsc-v1.2.0

    # When promoting to stable Tsc-s1.2.0
    git tag Tsc-s1.2.0
    git push origin Tsc-s1.2.0

Both tags point to the same commit (same feature, different status).

---

## Why This Scheme?

| Benefit | Explanation |
|---------|-------------|
| Quality enforced | Features do not pile up before being stable |
| Bugs do not accumulate | Tsc-v phase focuses on fixing bugs |
| Clear for contributors | Contributors know: in Tsc-v, do not add features |
| Stable before moving on | New features build on a solid foundation |
| Solo-dev friendly | Forces discipline, no jumping around |

### Trade-offs

| Trade-off | Impact |
|-----------|--------|
| Slower releases | Each feature needs 2 releases: Tsc-v then Tsc-s |
| More versions | 1 feature = 2 versions (v + s) |
| Requires testing | Tsc-v phase needs tests to verify stability |
| Requires dogfooding | You must use it yourself to find bugs |