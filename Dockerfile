#FROM rust:latest AS build
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest
FROM ${BASE_IMAGE} AS builder

COPY * ./
ADD --chown=rust:rust . ./
RUN cargo build --release

FROM alpine:latest
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/pok-server /usr/local/bin
COPY .env /home/rust
CMD /usr/local/bin/pok-server
