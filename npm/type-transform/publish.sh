#!/usr/bin/env bash

set -e

cd ./type-transform-linux-x64
npm publish

cd ../type-transform-darwin-arm64
npm publish

cd ../type-transform-darwin-x64
npm publish

cd ../type-transform-windows-x64
npm publish

cd ../type-transform

npm version 1.0.0 --no-git-tag-version

npm publish
