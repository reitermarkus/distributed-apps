FROM ekidd/rust-musl-builder:1.48.0 as builder

ARG BIN_NAME
ARG PROFILE

COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo install --path . ${PROFILE:+--${PROFILE}} --bin "${BIN_NAME}" --root /home/rust

FROM scratch

ARG BIN_NAME

COPY --from=builder "/home/rust/bin/${BIN_NAME}" /usr/bin/main

CMD ["main"]
