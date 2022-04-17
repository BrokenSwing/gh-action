#!/usr/bin/env bash
echo "TODO implement this script."
echo "It should build binaries in dist/<platform>-<arch>[.exe] as needed."
cargo build
mkdir dist
cp target/debug/gh-action.exe dist/windows-amd64.exe