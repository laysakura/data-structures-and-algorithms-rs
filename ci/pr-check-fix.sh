#!/usr/bin/env bash
set -eux

travis_terminate() {
    set +e
    pkill -9 -P $$ &> /dev/null || true
    exit $1
}

rustup component add rustfmt
cargo fmt --all -- --check || travis_terminate 1 || :
