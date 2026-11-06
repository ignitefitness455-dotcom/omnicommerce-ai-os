# ============================================================================
# Stage 1: Rust Workspace Builder (Multi-Stage Production Build)
# ============================================================================
FROM rust:1.82-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests & source
COPY Cargo.toml ./
COPY crates ./crates
COPY apps/server ./apps/server

RUN cargo build --release -p omni-server

# ============================================================================
# Stage 2: Minimal Non-Root Production Runtime
# ============================================================================
FROM debian:bookworm-slim AS runner

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system --gid 1001 omni \
    && useradd --system --uid 1001 --gid omni omniuser

COPY --from=builder /app/target/release/omni-server /usr/local/bin/omni-server

USER omniuser

ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080
EXPOSE 8080

HEALTHCHECK --interval=15s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

CMD ["omni-server"]
