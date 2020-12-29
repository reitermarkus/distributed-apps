#!/usr/bin/env bash

set -euo pipefail

name="${1}"
archive="target/${name}_rs.zip"

rust-musl-builder() {
  docker run --rm \
    -v "$(pwd)":/home/rust/src \
    -v cargo-git:/home/rust/.cargo/git \
    -v cargo-registry:/home/rust/.cargo/registry \
    ekidd/rust-musl-builder:1.48.0 \
    "${@}"
}

rust-musl-builder sudo chown -R rust:rust /home/rust/.cargo/git /home/rust/.cargo/registry 1>&2
rust-musl-builder cargo build --bin "${name}" --release 1>&2

mkdir -p "${archive}.d"
cp -f "target/x86_64-unknown-linux-musl/release/${name}" "${archive}.d/exec" 1>&2
zip -9 -FS -m -j "${archive}" "${archive}.d/exec" 1>&2
rm -rf "${archive}.d"

echo "{\"filename\":\"${archive}\",\"id\":\"$(date +%s)\"}"
