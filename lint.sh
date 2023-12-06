#!/usr/bin/env bash

pushd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &>/dev/null || exit 1

cargo +nightly clippy --all-targets --all-features --fix --allow-dirty &>/dev/null || true
cargo +nightly fmt || exit 1
cargo clippy --all-targets --all-features -- -D warnings || exit 1
