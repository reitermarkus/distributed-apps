#!/bin/sh

set -euo pipefail

name="${1}"
archive="target/${name}_rs.zip"

alias rust-musl-builder='docker run --rm -v "$(pwd)":/home/rust/src -v cargo-git:/home/rust/.cargo/git -v cargo-registry:/home/rust/.cargo/registry ekidd/rust-musl-builder:1.48.0'

rust-musl-builder sudo chown -R rust:rust /home/rust/.cargo/git /home/rust/.cargo/registry 1>&2
rust-musl-builder cargo build --bin "${name}" --release 1>&2
cp -f "target/x86_64-unknown-linux-musl/release/${name}" exec 1>&2
zip -9 -mj "${archive}" exec 1>&2

echo "{\"filename\":\"${archive}\",\"id\":\"$(date +%s)\"}"
