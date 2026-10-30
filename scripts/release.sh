#!/usr/bin/env bash
# Usage: ./scripts/release.sh 1.0.8
set -eo pipefail

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 1.0.8"
    exit 1
fi

# 1. Update Cargo.toml
echo "Updating Cargo.toml to $VERSION..."
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# 2. Update VERSION file
echo "Kayls-v$VERSION" > VERSION

# 3. Update Cargo.lock
echo "Updating Cargo.lock..."
if ! cargo update -w; then
    echo "ERROR: cargo update -w failed. Cargo.lock may be stale."
    exit 1
fi

# 3. Commit
echo "Committing version bump..."
git add Cargo.toml VERSION Cargo.lock
git commit -m "chore: release v$VERSION"

# Konfirmasi sebelum push
read -p "Push to main and create tag v$VERSION? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Buat branch rilis
BRANCH="release/v$VERSION"
git checkout -b "$BRANCH"
git push origin "$BRANCH"

# Buat PR (menggunakan gh)
if command -v gh >/dev/null 2>&1; then
    gh pr create --title "chore: release v$VERSION" --body "Release v$VERSION" --base main --head "$BRANCH"
    echo "PR created. Merge it to trigger the release."
else
    echo "gh CLI not found. Please create PR manually: https://github.com/$REPO/compare/main...$BRANCH"
fi

# Tag tetap dibuat di main setelah PR di-merge (manual)
echo "After merging, run: git tag Kayls-v$VERSION && git push origin Kayls-v$VERSION"

echo "✅ PR created for v$VERSION. Merge it, then tag manually."
echo "Monitor at: https://github.com/iyjaa/Kayls/actions"
