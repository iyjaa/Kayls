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

git push origin main

# 4. Create tag
echo "Creating tag Kayls-v$VERSION..."
git tag "Kayls-v$VERSION"
git push origin "Kayls-v$VERSION"

echo "✅ Release v$VERSION triggered!"
echo "Monitor at: https://github.com/iyjaa/Kayls/actions"
