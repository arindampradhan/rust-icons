.PHONY: help fmt check lint test build clean dev serve

help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

fmt: ## Format all Rust code with rustfmt
	cargo fmt --all

check: ## Check if code is formatted correctly
	cargo fmt --all -- --check

lint: ## Run clippy lints
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test: ## Run all tests
	cargo test --workspace --all-features

build: ## Build all crates in release mode
	cargo build --workspace --all-features --release

clean: ## Clean build artifacts
	cargo clean

dev: ## Start Trunk dev server for leptos-icons (port 4000)
	cd crates/leptos-icons && trunk serve

precheck: ## Run formatting, linting, and tests
	@echo "Checking formatting..."
	@cargo fmt --all -- --check
	@echo "Running clippy..."
	@cargo clippy --workspace --all-targets --all-features -- -D warnings
	@echo "Running tests..."
	@cargo test --workspace --all-features
	@echo "✓ All checks passed!"

ci: precheck ## Run CI checks locally (same as precheck)
