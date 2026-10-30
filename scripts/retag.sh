#!/usr/bin/env bash
# Usage: ./scripts/retag.sh 1.0.10
set -eo pipefail

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 1.0.10"
    exit 1
fi

# Pastikan di branch main
BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$BRANCH" != "main" ]; then
    echo "ERROR: Must be on 'main' branch (currently on '$BRANCH')."
    exit 1
fi

# Pastikan sinkron dengan origin/main
git fetch origin main
LOCAL=$(git rev-parse HEAD)
REMOTE=$(git rev-parse origin/main)
if [ "$LOCAL" != "$REMOTE" ]; then
    echo "ERROR: Local main is not synced with origin/main."
    echo "Run: git pull origin main"
    exit 1
fi

TAG="Kayls-v$VERSION"

echo "Deleting old tag $TAG (local and remote)..."
git tag -d "$TAG" 2>/dev/null || true
git push origin ":refs/tags/$TAG" 2>/dev/null || true

echo "Creating new tag $TAG at HEAD..."
git tag "$TAG"

echo "Pushing tag $TAG to trigger release..."
git push origin "$TAG"

echo "✅ Retag $TAG triggered!"
echo "Monitor at: https://github.com/iyjaa/Kayls/actions"
