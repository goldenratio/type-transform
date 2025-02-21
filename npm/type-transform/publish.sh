#!/usr/bin/env bash

set -euo pipefail

# Ensure a version argument is provided
if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <version>"
  exit 1
fi

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
