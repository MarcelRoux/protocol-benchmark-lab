FROM rust:1.95-slim-bullseye AS builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY services/rust/Cargo.toml services/rust/
RUN mkdir -p services/rust/src
COPY services/rust/src services/rust/src
RUN cargo fetch
RUN cargo build --release --package rust

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/rust /usr/local/bin/rust
EXPOSE 3000
USER 1000
CMD ["/usr/local/bin/rust"]