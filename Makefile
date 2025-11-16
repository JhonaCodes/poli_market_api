.PHONY: help build run test clean docker-build docker-up docker-down docker-logs

help:
	@echo "PoliMarket API - Makefile Commands"
	@echo ""
	@echo "Development:"
	@echo "  make build         - Build the project"
	@echo "  make run           - Run the project"
	@echo "  make test          - Run tests"
	@echo "  make clean         - Clean build artifacts"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-build  - Build Docker images"
	@echo "  make docker-up     - Start Docker services"
	@echo "  make docker-down   - Stop Docker services"
	@echo "  make docker-logs   - View Docker logs"
	@echo ""

build:
	@echo "Building project..."
	cargo build --release

run:
	@echo "Running project..."
	cargo run

test:
	@echo "Running tests..."
	cargo test

clean:
	@echo "Cleaning build artifacts..."
	cargo clean

docker-build:
	@echo "Building Docker images..."
	docker-compose build

docker-up:
	@echo "Starting Docker services..."
	docker-compose up -d
	@echo "API running at http://localhost:8080"
	@echo "PostgreSQL running at localhost:5432"

docker-down:
	@echo "Stopping Docker services..."
	docker-compose down

docker-logs:
	@echo "Viewing Docker logs..."
	docker-compose logs -f api

format:
	@echo "Formatting code..."
	cargo fmt

check:
	@echo "Checking code..."
	cargo check

clippy:
	@echo "Running Clippy..."
	cargo clippy
