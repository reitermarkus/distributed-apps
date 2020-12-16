#!/bin/sh

set -euo pipefail

build() {
  docker run -it -v "$(pwd):/home/rust/src" ekidd/rust-musl-builder:1.48.0 cargo build --bin "${1}" --release
  cp -f "target/x86_64-unknown-linux-musl/release/${1}" exec
  zip "target/${1}.zip" exec
  rm -f exec
}

build fetch_prices
build forecast

deploy() {
  ibmcloud fn action delete "${1}" || true
  ibmcloud fn action create "${1}" --native "target/${1}.zip"
}

deploy fetch_prices
deploy forecast
