# Development Guide

## Getting Started

### Prerequisites

```bash
# Check versions
rust --version    # 1.70+
go version         # 1.21+
flutter --version  # 3.13+
node --version     # 18+
psql --version     # 15+
```

### Initial Setup

```bash
git clone https://github.com/Rafayalmani/nexus.git
cd nexus

# Start database
docker-compose up -d postgres

# Run migrations
psql -h localhost -U nexus -d nexus -f services/coordinator/migrations/001_initial.sql
```

## Building Components

### Coordinator

```bash
cd services/coordinator
go mod download
go run main.go

# Or build binary
go build -o coordinator main.go
./coordinator
```

**Endpoints:**
- `GET /health` - Health check
- `POST /devices/register` - Register device
- `WS /presence` - WebSocket for online status

### Sync Engine

```bash
cd crates/sync-engine

# Run example
cargo run --example watch-folder -- /path/to/watch

# Run tests
cargo test

# Build library
cargo build --release
```

### Crypto

```bash
cd crates/crypto
cargo test
cargo build --release
```

### Networking

```bash
cd crates/networking
cargo build --release
```

### Desktop App

```bash
cd apps/desktop
flutter pub get
flutter run -d windows
# or: macos, linux, chrome
```

## Testing

### Rust

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p sync-engine

# With output
cargo test -- --nocapture
```

### Go

```bash
cd services/coordinator
go test ./...
go test -v ./...
```

### Flutter

```bash
cd apps/desktop
flutter test
```

## Code Style

### Rust

```bash
# Format
cargo fmt --all

# Lint
cargo clippy --all
```

### Go

```bash
# Format
go fmt ./...

# Lint
golangci-lint run
```

### Dart

```bash
cd apps/desktop
flutter analyze
dart format lib/
```

## Debugging

### Enable Logging

**Rust:**
```rust
tracing_subscriber::fmt::init();
```

**Go:**
```go
log.Printf("message: %v", value)
```

## Contributing

1. Create feature branch: `git checkout -b feature/my-feature`
2. Make changes following code style
3. Run tests: `make test`
4. Commit: `git commit -am 'Add feature'`
5. Push: `git push origin feature/my-feature`
6. Create Pull Request

## Phase Milestones

### Phase 1 - Foundation (Current)
- [ ] Coordinator server
- [ ] Sync engine
- [ ] File watching
- [ ] Chunking & hashing
- [ ] Flutter desktop shell
- [ ] Device pairing
- [ ] Encryption layer
- [ ] Local sync between 2 devices

### Phase 2 - P2P + Relay
- [ ] STUN client
- [ ] UDP hole punching
- [ ] QUIC transport
- [ ] Relay server
- [ ] LAN discovery (mDNS)
- [ ] Cross-internet sync

### Phase 3 - Full Features
- [ ] On-demand file access
- [ ] Clipboard sync
- [ ] VS Code extension
- [ ] Browser extension
- [ ] Version history UI
- [ ] Device dashboard
- [ ] Mobile app
- [ ] Photo/video streaming
