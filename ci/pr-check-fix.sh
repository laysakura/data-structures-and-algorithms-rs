#!/usr/bin/env bash
set -eux

travis_terminate() {
    set +e
    pkill -9 -P $$ &> /dev/null || true
    exit $1
}

rustup component add rustfmt

## Auto commit & push by CI
(
    cd `mktemp -d`
    git clone https://github.com/${TRAVIS_REPO_SLUG}.git
    cd data-structures-and-algorithms-rs
    git checkout ${TRAVIS_PULL_REQUEST_BRANCH}

    committed=0

    ### cargo fmt
    cargo fmt --all -- --check || travis_terminate 1 || :
)
