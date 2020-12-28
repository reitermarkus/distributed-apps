#!/bin/sh

set -euo pipefail

pushd "$(dirname "${0}")"

alias rust-musl-builder='docker run --rm -v "$(pwd)":/home/rust/src -v cargo-git:/home/rust/.cargo/git -v cargo-registry:/home/rust/.cargo/registry -v target:/home/rust/src/target ekidd/rust-musl-builder:1.48.0'

rust-musl-builder sudo chown -R rust:rust \
  /home/rust/.cargo/git /home/rust/.cargo/registry /home/rust/src/target
rust-musl-builder cargo build --bin "${1}" --release
cp -f "target/x86_64-unknown-linux-musl/release/${1}" exec
zip "target/${1}_rs.zip" exec
rm -f exec

ibmcloud login -r us-east -g Default --apikey @ibmcloud_api_key.txt
ibmcloud fn namespace target "${2}"
ibmcloud fn action delete "${1}_rs" || true
ibmcloud fn action create "${1}_rs" --timeout 10000 --memory 128 --native "target/${1}_rs.zip"
