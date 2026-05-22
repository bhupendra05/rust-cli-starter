# ── Stage 1: Build ────────────────────────────────────────────────────────────
FROM rust:1.78-slim AS builder

WORKDIR /app

# Cache dependencies separately from source
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/mycli*

# Build the real binary
COPY src ./src
RUN cargo build --release

# ── Stage 2: Runtime ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Non-root user
RUN groupadd -r mycli && useradd -r -g mycli mycli

COPY --from=builder /app/target/release/mycli /usr/local/bin/mycli

USER mycli
ENTRYPOINT ["mycli"]
