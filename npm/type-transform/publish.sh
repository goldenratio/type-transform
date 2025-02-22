#!/usr/bin/env bash

set -euo pipefail

# Get the latest Git tag
VERSION=$(git describe --tags --abbrev=0 2>/dev/null || echo "")

if [[ -z "$VERSION" ]]; then
  echo "No Git tags found. Please create a tag first."
  exit 1
fi

# Strip the 'v' prefix if present
VERSION=${VERSION#v}

echo "Publishing Version: $VERSION"

read -p "Are you sure you want to publish this version? (y/N): " CONFIRM

if [[ "$CONFIRM" != "y" ]]; then
  echo "Publishing canceled."
  exit 0
fi

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

# Update dependencies version
for PLATFORM in "${PLATFORMS[@]}"; do
  npm install "@goldenratio/$PLATFORM@$VERSION" --save-exact --save-optional
done

npm install
npm version "$VERSION" --no-git-tag-version
npm publish

git add .
git commit -m "chore: release NPM version ${VERSION}"
git push
