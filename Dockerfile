FROM rust:1.92 AS base
WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

FROM base AS builder-linux
RUN rustup target add x86_64-unknown-linux-gnu && \
    cargo build --release --target x86_64-unknown-linux-gnu && \
    strip target/x86_64-unknown-linux-gnu/release/kuack-checker

FROM base AS builder-wasm
RUN cargo install wasm-pack
RUN wasm-pack build --target web --release

FROM gcr.io/distroless/cc-debian12:nonroot AS runtime-linux
COPY --from=builder-linux /build/target/x86_64-unknown-linux-gnu/release/kuack-checker /kuack-checker
ENTRYPOINT ["/kuack-checker"]

FROM scratch AS runtime-wasm
COPY --from=builder-wasm /build/pkg /pkg

FROM runtime-linux AS default
