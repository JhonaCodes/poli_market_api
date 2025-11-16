# Multi-stage build for Rust application
# Optimized for production deployment with proper error handling

# Stage 1: Builder
FROM rust:1.90 AS builder

WORKDIR /app

# Copy manifests first for better layer caching
COPY Cargo.toml ./

# Copy Cargo.lock if it exists (optional but recommended)
# Using shell test to avoid build failure if file doesn't exist
RUN mkdir -p .cargo
COPY Cargo.lock* ./

# Create a dummy main.rs to cache dependencies
# This layer will be cached unless Cargo.toml changes
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code and configuration
COPY src ./src
COPY diesel.toml ./
COPY migrations ./migrations

# Build the actual application
# Use --locked if Cargo.lock exists to ensure reproducible builds
RUN cargo build --release && \
    strip target/release/poli_market_api

# Verify the binary was created and is executable
RUN test -f target/release/poli_market_api && \
    test -x target/release/poli_market_api

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
# - libpq5: PostgreSQL client library
# - ca-certificates: For HTTPS connections
# - curl: For healthcheck
RUN apt-get update && \
    apt-get install -y \
    libpq5 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/poli_market_api .

# Copy entrypoint script
COPY docker-entrypoint.sh .

# Ensure binary and script are executable
RUN chmod +x ./poli_market_api && \
    chmod +x ./docker-entrypoint.sh

# Create non-root user for security
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

USER appuser

# Expose the application port
EXPOSE 8080

# Add healthcheck to ensure container is actually running
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8080/v1/health || exit 1

# Use entrypoint for better debugging
ENTRYPOINT ["./docker-entrypoint.sh"]

# Run the application
CMD ["./poli_market_api"]
