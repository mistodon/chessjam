#!/bin/bash

set -e

function dothing {
    echo -e "\033[1;36m$@\033[0m ... "
    $@
    echo -e "\033[1;36m$@\033[0m done."
}

export RUSTFLAGS="-D warnings"

dothing cargo check --release
dothing cargo build
dothing cargo test
dothing cargo +nightly fmt --all -- --write-mode=diff
dothing cargo +nightly clippy -- -D clippy
