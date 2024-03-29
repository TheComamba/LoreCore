#!/bin/bash
set -e

# Go to the root directory of the Git repository
cd "$(git rev-parse --show-toplevel)"

if ! command -v diesel &> /dev/null; then
    cargo install diesel_cli --no-default-features --features sqlite-bundled
fi

diesel setup --database-url=dummy.db
diesel print-schema --database-url=dummy.db > src/sql/schema.rs
rm dummy.db
cargo fmt
