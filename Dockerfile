# Multi-stage Dockerfile for Meridian RS
# Supports building core-only or with integrated Next.js web UI
# Build: docker build --build-arg WEB_ENABLED=true -t meridian:web .

# === Stage 1: Web UI builder (conditional) ===
FROM node:22-alpine AS web-builder
ARG WEB_ENABLED=false
ARG WEB_BRANCH=web-ui
ARG GIT_REPO_URL=https://github.com/ilhaammulia/meridian-rs.git

WORKDIR /build

# Only run if web UI is enabled
RUN if [ "$WEB_ENABLED" = "true" ]; then \
    apk add --no-cache git && \
    git clone --depth 1 --branch $WEB_BRANCH --single-branch \
    $GIT_REPO_URL . && \
    if [ -f package.json ]; then npm ci && npm run build; fi \
    ; fi

# === Stage 2: Rust builder ===
FROM rust:1.82 AS rust-builder

ARG CORE_BRANCH=master
ARG WEB_ENABLED=false
ARG GIT_REPO_URL=https://github.com/ilhaammulia/meridian-rs.git

WORKDIR /build

# Clone core backend
RUN git clone --depth 1 --branch $CORE_BRANCH --single-branch \
    $GIT_REPO_URL .

# Build Rust binary
RUN cargo build --release 2>&1 && \
    mv /build/target/release/meridian-rs /build/meridian-rs

# === Stage 3: Runtime ===
FROM alpine:3.20

RUN apk add --no-cache \
    ca-certificates \
    curl \
    bash \
    libssl3

WORKDIR /app

# Copy binary and example configs
COPY --from=rust-builder /build/meridian-rs /app/meridian-rs
COPY --from=rust-builder /build/.env.example /app/.env.example
COPY --from=rust-builder /build/user-config.example.json /app/user-config.example.json

# Create data directory
RUN mkdir -p /data && chmod 755 /data

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Runtime environment variables
ENV MERIDIAN_DATA_DIR=/data \
    MERIDIAN_LOG_STYLE=pretty \
    MERIDIAN_WEB_ADDR=0.0.0.0:3000 \
    HEALTH_PORT=8080

# Expose ports
EXPOSE 3000 8080

# Entrypoint script
RUN echo '#!/bin/sh' > /app/entrypoint.sh && \
    echo 'set -e' >> /app/entrypoint.sh && \
    echo '' >> /app/entrypoint.sh && \
    echo '# Check if .env exists' >> /app/entrypoint.sh && \
    echo 'if [ ! -f "$MERIDIAN_DATA_DIR/.env" ]; then' >> /app/entrypoint.sh && \
    echo '    echo "⚠ .env not found in $MERIDIAN_DATA_DIR"' >> /app/entrypoint.sh && \
    echo '    echo "Copying example .env..."' >> /app/entrypoint.sh && \
    echo '    cp /app/.env.example "$MERIDIAN_DATA_DIR/.env"' >> /app/entrypoint.sh && \
    echo '    echo "📝 Edit $MERIDIAN_DATA_DIR/.env with your configuration"' >> /app/entrypoint.sh && \
    echo 'fi' >> /app/entrypoint.sh && \
    echo '' >> /app/entrypoint.sh && \
    echo '# Check if user-config.json exists' >> /app/entrypoint.sh && \
    echo 'if [ ! -f "$MERIDIAN_DATA_DIR/user-config.json" ]; then' >> /app/entrypoint.sh && \
    echo '    echo "⚠ user-config.json not found in $MERIDIAN_DATA_DIR"' >> /app/entrypoint.sh && \
    echo '    echo "Copying example user-config.json..."' >> /app/entrypoint.sh && \
    echo '    cp /app/user-config.example.json "$MERIDIAN_DATA_DIR/user-config.json"' >> /app/entrypoint.sh && \
    echo '    echo "📝 Edit $MERIDIAN_DATA_DIR/user-config.json with your configuration"' >> /app/entrypoint.sh && \
    echo 'fi' >> /app/entrypoint.sh && \
    echo '' >> /app/entrypoint.sh && \
    echo 'exec /app/meridian-rs "$@"' >> /app/entrypoint.sh && \
    chmod +x /app/entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]
CMD []
