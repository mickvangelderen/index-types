#!/usr/bin/env bash

pushd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &>/dev/null || exit 1

./lint.sh || exit 1
cargo test --all-features || exit 1
