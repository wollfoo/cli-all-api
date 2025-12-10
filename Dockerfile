# =============================================================================
# ProxyPal Headless - Multi-stage Dockerfile
# Container deployment cho AI proxy server
# =============================================================================

# -----------------------------------------------------------------------------
# Stage 1: Builder - Compile Rust binary
# -----------------------------------------------------------------------------
FROM rust:1.83-bookworm AS builder

# Cài đặt dependencies cho build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo files trước để cache dependencies
COPY src-headless/Cargo.toml src-headless/Cargo.lock* ./

# Tạo dummy main.rs để build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release || true && \
    rm -rf src

# Copy source code thực
COPY src-headless/src ./src

# Build release binary
RUN cargo build --release && \
    strip target/release/proxypal

# -----------------------------------------------------------------------------
# Stage 2: Runtime - Minimal image với binaries
# -----------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

# Cài đặt runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/false proxypal

# Tạo directories cho config và auth
RUN mkdir -p /etc/proxypal \
    /var/lib/proxypal/auth \
    /var/log/proxypal \
    && chown -R proxypal:proxypal /etc/proxypal /var/lib/proxypal /var/log/proxypal

# Copy ProxyPal binary từ builder stage
COPY --from=builder /app/target/release/proxypal /usr/local/bin/proxypal
RUN chmod +x /usr/local/bin/proxypal

# Copy CLIProxyAPI binary (build arg để linh hoạt)
ARG CLIPROXYAPI_BINARY=src-tauri/binaries/cliproxyapi-x86_64-unknown-linux-gnu
COPY ${CLIPROXYAPI_BINARY} /usr/local/bin/cliproxyapi
RUN chmod +x /usr/local/bin/cliproxyapi

# Environment variables
ENV PROXYPAL_CONFIG=/etc/proxypal/config.yaml
ENV PROXYPAL_AUTH_DIR=/var/lib/proxypal/auth
ENV PROXYPAL_LOG_DIR=/var/log/proxypal
ENV RUST_LOG=info

# Expose port (default ProxyPal port)
EXPOSE 8317

# Volume mounts
VOLUME ["/etc/proxypal", "/var/lib/proxypal/auth", "/var/log/proxypal"]

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8317/health || proxypal check --port 8317 || exit 1

# Run as non-root user
USER proxypal

# Default entrypoint và command
ENTRYPOINT ["proxypal"]
CMD ["serve", "--foreground", "--config", "/etc/proxypal/config.yaml"]

# -----------------------------------------------------------------------------
# Labels
# -----------------------------------------------------------------------------
LABEL org.opencontainers.image.title="ProxyPal Headless"
LABEL org.opencontainers.image.description="AI Proxy Server for Coding Agents"
LABEL org.opencontainers.image.version="0.1.0"
LABEL org.opencontainers.image.source="https://github.com/wollfoo/cli-all-api"
LABEL org.opencontainers.image.licenses="MIT"
