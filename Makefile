.PHONY: help build test lint clean run-coordinator run-sync-engine

help:
	@echo "Nexus P2P Build System"
	@echo "======================"
	@echo "make build-all          - Build all components"
	@echo "make build-coordinator  - Build coordinator service"
	@echo "make build-sync-engine  - Build Rust sync engine"
	@echo "make build-crypto       - Build crypto crate"
	@echo "make build-networking   - Build networking crate"
	@echo "make build-desktop      - Build Flutter desktop app"
	@echo "make test               - Run all tests"
	@echo "make lint               - Run linters"
	@echo "make clean              - Clean build artifacts"

# Build targets
build-all: build-coordinator build-sync-engine build-crypto build-networking build-desktop

build-coordinator:
	cd services/coordinator && go build -o ../../bin/coordinator main.go

build-sync-engine:
	cd crates/sync-engine && cargo build --release

build-crypto:
	cd crates/crypto && cargo build --release

build-networking:
	cd crates/networking && cargo build --release

build-desktop:
	cd apps/desktop && flutter build windows

# Development targets
run-coordinator:
	cd services/coordinator && go run main.go

run-sync-engine:
	cd crates/sync-engine && cargo run --example watch-folder -- .

# Testing
test:
	cargo test --workspace

test-coordinator:
	cd services/coordinator && go test ./...

# Linting
lint:
	cargo clippy --workspace
	@echo "Rust linting complete"

# Cleanup
clean:
	cargo clean
	cd services/coordinator && rm -f coordinator
	rm -rf bin/
