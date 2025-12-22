.PHONY: help build run run-debug run-trace run-release clean dev test stop logs

# Default target
help:
	@echo ""
	@echo "Available commands:"
	@echo "  make build          - Build the project in debug mode"
	@echo "  make build-release  - Build the project in release mode"
	@echo "  make run            - Run server with INFO level logging"
	@echo "  make dev            - Run server with DEBUG level logging"
	@echo "  make run-trace      - Run server with TRACE level logging"
	@echo "  make run-release    - Run server in release mode with WARN logging"
	@echo "  make test           - Run tests"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make stop           - Stop the server (kill process on port 4221)"
	@echo "  make logs           - Show recent logs"
	@echo ""

# Build the project
build:
	@echo "Building project..."
	cargo build

# Build in release mode
build-release:
	@echo "Building project in release mode..."
	cargo build --release

# Run with INFO level (default)
run: build
	@echo "Starting server with INFO logging..."
	RUST_LOG=info cargo run

# Run with DEBUG level (development)
dev: build
	@echo "Starting server with DEBUG logging..."
	RUST_LOG=debug cargo run

# Run with TRACE level (verbose debugging)
run-trace: build
	@echo "Starting server with TRACE logging..."
	RUST_LOG=trace cargo run

# Run in release mode with minimal logging
run-release: build-release
	@echo "Starting server in release mode..."
	RUST_LOG=warn cargo run --release

# Run with custom log level
run-custom:
	@echo "Starting server with custom log level: $(LEVEL)"
	RUST_LOG=$(LEVEL) cargo run

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Stop the server (works on Unix-like systems)
stop:
	@echo "Stopping server on port 4221..."
	@lsof -ti:4221 | xargs kill -9 2>/dev/null || echo "No process found on port 4221"

# Watch logs (if you're using file logging)
logs:
	@echo "Recent server activity:"
	@tail -f server.log 2>/dev/null || echo "No log file found"

# Development mode with auto-reload (requires cargo-watch)
watch:
	@echo "Starting server with auto-reload..."
	@if command -v cargo-watch >/dev/null 2>&1; then \
		RUST_LOG=debug cargo watch -x 'run'; \
	else \
		echo "cargo-watch not found. Install it with: cargo install cargo-watch"; \
		exit 1; \
	fi
# Check code without building
check:
	@echo "Checking code..."
	cargo check

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Run clippy linter
lint:
	@echo "Running clippy..."
	cargo clippy -- -D warnings
