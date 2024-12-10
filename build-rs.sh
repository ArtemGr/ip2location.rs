#!/bin/bash
PATH=$PATH:$HOME/.cargo/bin
set -e
cargo build -vv
cargo test
