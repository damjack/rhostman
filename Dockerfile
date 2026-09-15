FROM rust:1.98.1-bookworm as builder

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.98.1

WORKDIR /code

RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME && \
    chown -R root:root /code && \
    cargo install cargo-make && \
    cargo install cargo-prune && \
    cargo install cargo-sweep && \
    cargo install cargo-watch && \
    cargo install cargo-make && \
    rustup component add clippy rustfmt

USER root
