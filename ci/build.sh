#!/bin/sh
cargo build --verbose --all --target "$TARGET"

case "$TARGET" in
    x86_64-unknown-linux-gnu)
        cargo test --verbose --all
        ;;
esac
