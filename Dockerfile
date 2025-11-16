# Build stage
FROM rust:1.90 AS builder

WORKDIR /app

# Install build dependencies (curl needed for utoipa-swagger-ui)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/poli_market_api .

ENV RUST_LOG=info

EXPOSE 8080

CMD ["./poli_market_api"]
