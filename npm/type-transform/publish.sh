#!/usr/bin/env bash

set -euo pipefail

VERSION="$1"

echo "Publishing Version: $VERSION"

# List of directories to publish
PLATFORMS=(
  "type-transform-linux-x64"
  "type-transform-darwin-arm64"
  "type-transform-darwin-x64"
  "type-transform-windows-x64"
)

# Publish each platform
for PLATFORM in "${PLATFORMS[@]}"; do
  echo "Publishing $PLATFORM..."
  (cd "../$PLATFORM" && npm publish)
done

# Update and publish the main package
cd ../type-transform
npm version "$VERSION" --no-git-tag-version
npm publish

git commit -m "core: release NPM version ${VERSION}"
git push
