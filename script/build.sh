#!/usr/bin/env bash
# "It should build binaries in dist/<platform>-<arch>[.exe] as needed."

cargo build
mkdir dist
cp target/release/gh-action.exe dist/windows-amd64.exe