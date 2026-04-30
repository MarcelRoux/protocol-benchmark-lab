FROM rust:1.95-slim-bullseye AS builder
WORKDIR /usr/src/app

# Workspace manifests - for dependency resolution.
COPY Cargo.toml Cargo.lock ./
COPY orchestrator/Cargo.toml orchestrator/
COPY services/rust/Cargo.toml services/rust/

# Workspace members must have at least one target for cargo metadata/fetch.
RUN mkdir -p orchestrator/src services/rust/src \
 && printf 'fn main() {}\n' > orchestrator/src/main.rs \
 && printf 'fn main() {}\n' > services/rust/src/main.rs

RUN cargo fetch

# Target binary.
COPY orchestrator/src orchestrator/src
RUN cargo build --release --package orchestrator

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/orchestrator /usr/local/bin/orchestrator
EXPOSE 3000
USER 1000
CMD ["/usr/local/bin/orchestrator"]