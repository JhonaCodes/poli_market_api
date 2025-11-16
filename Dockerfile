# Multi-stage build for Rust application
FROM rust:1.90-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source and migrations
COPY src ./src
COPY diesel.toml ./
COPY migrations ./migrations

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    postgresql-client \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary and startup script
COPY --from=builder /app/target/release/poli_market_api /app/
COPY ./start.sh /app/start.sh
COPY ./migrations /app/migrations
COPY ./diesel.toml /app/

# Make executable
RUN chmod +x /app/start.sh /app/poli_market_api

# Create non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

USER appuser

# Set defaults
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \
  CMD curl -f http://localhost:8080/v1/health || exit 1

# Start with script
CMD ["/app/start.sh"]
