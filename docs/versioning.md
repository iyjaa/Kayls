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

- X=1: Editor integration (Tsc-v1.0.0)
- X=2: Integrated terminal
- X=3: LSP
- X=4: AI agent

X is planned ahead in the roadmap.

### Y — Sub-feature

A sub-feature inside a major feature. The number of Y is **not known in advance**. It is discovered while building the feature, based on the minimum requirements for it to work.

Example for X=2 (Integrated terminal):

| Y | Sub-feature | Minimum to Work |
|---|-------------|-----------------|
| 1 | Spawn shell | Can start a shell process |
| 2 | Send input | Can type into the shell |
| 3 | Display output | Can see shell output |
| 4 | Resize terminal | Terminal adapts to window size |
| 5 | Multiple tabs | Can open multiple terminals |

If X=2 turns out to need 4 sub-features, it finishes at Y=4. If it needs 7, it finishes at Y=7.

### Z — Bug Fix Count

How many times a bug has been fixed at X.Y. Starts at 0.

Example:

- Tsc-v2.1.0 — sub-feature 1 done, no bugs yet
- Tsc-v2.1.1 — bug found, fixed (fix #1)
- Tsc-v2.1.2 — another bug fixed (fix #2)
- Tsc-s2.1.2 — stable after 3 days of dogfooding

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

### Example 1: Editor Integration (X=1)

    Tsc-v1.0.0   start X=1, editor integration
    Tsc-v1.0.1   bug fix in editor
    Tsc-s1.0.1   stable after 3 days dogfooding
    Tsc-v1.1.0   sub-feature 1 of X=1 (cursor improvements)
    Tsc-s1.1.0   stable
    ...

### Example 2: Integrated Terminal (X=2)

    Tsc-v2.0.0   start X=2, terminal integration
    Tsc-v2.1.0   sub-feature 1 done (spawn shell)
    Tsc-v2.1.1   bug in spawn shell fixed
    Tsc-s2.1.1   sub-feature 1 stable
    Tsc-v2.2.0   sub-feature 2 done (send input)
    Tsc-s2.2.0   sub-feature 2 stable
    ...
    Tsc-s2.5.0   all sub-features done, stable
    Tsc-v3.0.0   start X=3 (LSP)

### Example 3: Bug in Stable Version

    Tsc-s2.5.0   stable
                 (bug discovered)
    Tsc-v2.5.1   bug fixed, prefix drops to v
    Tsc-s2.5.1   stable again after 3 days

---

## Cargo.toml Workaround

Cargo.toml only accepts numeric X.Y.Z. The full version with prefix lives in a VERSION file at the repo root.

VERSION file:

    Tsc-v1.0.0

Cargo.toml:

    [workspace.package]
    version = "1.0.0"    # numeric only

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

Output: Tscode Tsc-v1.0.0

---

## Git Tags

Two tags per version:

    # When releasing Tsc-v1.0.0
    git tag Tsc-v1.0.0
    git push origin Tsc-v1.0.0

    # When promoting to stable Tsc-s1.0.0
    git tag Tsc-s1.0.0
    git push origin Tsc-s1.0.0

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

---

## Notes

- X is planned, Y is discovered, Z is counted
- A version can be released multiple times as Z increments
- When promoting to stable, prefix changes from `v` to `s` (same X.Y.Z)
- When a bug is found in stable, prefix changes from `s` to `v`, Z increments
- The `VERSION` file is the single source of truth for the full version string
- `Cargo.toml` keeps only the numeric part