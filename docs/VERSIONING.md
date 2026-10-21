# Kayls Versioning

Kayls uses a custom versioning scheme to enforce discipline and quality.

## Tag Format

Every release is tagged using the following format:

`Kayls-<type><version>`

Where:
- `<type>` is either `v` (stable) or `s` (snapshot/prerelease).
- `<version>` is the `MAJOR.MINOR.PATCH` number.

**Examples:**
- `Kayls-v1.0.0` (Stable release)
- `Kayls-s1.1.0-beta.1` (Snapshot / Pre-release)
- `Kayls-v1.1.0` (Minor update)

## Semantic Versioning Rules

Since Kayls is a terminal application:
- **MAJOR**: Incompatible changes, breaking CLI flags, or config file structure changes.
- **MINOR**: New features, new themes, new keybindings (backwards compatible).
- **PATCH**: Bug fixes, performance improvements, documentation updates.

## Release Process

Because the CI/CD pipeline is fully automated:
1. Maintainer creates a new tag (e.g., `git tag Kayls-v1.0.1`).
2. Maintainer pushes the tag (`git push origin Kayls-v1.0.1`).
3. GitHub Actions automatically builds binaries, creates `.deb` packages, updates the APT repository, and creates a GitHub Release.
