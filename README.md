# Nexus P2P Personal Sync Network

A decentralized workspace synchronization platform where user devices collectively form the 'cloud'. Direct P2P transfers with encrypted relay fallback.

## Quick Start

### Prerequisites
- Rust 1.70+
- Go 1.21+
- Flutter 3.13+
- Node.js 18+
- PostgreSQL 15+

### Phase 1 - Foundation (Current)

```bash
# Clone repo
git clone https://github.com/Rafayalmani/nexus.git
cd nexus

# Build coordinator
cd services/coordinator
go mod download
go build -o coordinator cmd/main.go

# Build sync engine
cd ../../crates/sync-engine
cargo build --release

# Build Flutter desktop
cd ../../apps/desktop
flutter pub get
flutter build windows  # or macos, linux
```

## Features

- ✅ File/folder sync with delta chunks
- ✅ Filesystem watching and smart ignore
- ✅ Device pairing (QR + 6-digit)
- ✅ End-to-end encryption (X25519, ChaCha20-Poly1305)
- 🔄 P2P + Relay (Phase 2)
- 🔄 Clipboard sync (Phase 3)
- 🔄 VS Code extension (Phase 3)
- 🔄 Browser extension (Phase 3)

## Architecture

```
[Desktop]     [Laptop]     [Phone]
    |            |           |
    +---- Coordinator ----+
           (metadata only)

Direct P2P when possible ←→
Relay (encrypted, TTL 24h) when not
```

## Project Structure

```
nexus/
├── apps/
│   ├── desktop/      # Flutter (Windows/macOS/Linux)
│   └── mobile/       # Flutter (Android/iOS)
├── services/
│   ├── coordinator/  # Go - device registry
│   └── relay/        # Go - encrypted relay
├── crates/
│   ├── sync-engine/  # Rust - file watching, chunking
│   ├── crypto/       # Rust - encryption
│   ├── networking/   # Rust - P2P, NAT traversal
│   └── filesystem/   # Rust - ignore rules
├── extensions/
│   ├── vscode/       # TypeScript - workspace sync
│   └── browser/      # TypeScript - tab sync
└── packages/
    └── protocol/     # Shared message formats
```

## Development

Each component can be developed independently:

### Coordinator
```bash
cd services/coordinator
go run main.go
```

### Sync Engine
```bash
cd crates/sync-engine
cargo run --example watch-folder -- /path/to/watch
```

### Flutter Desktop
```bash
cd apps/desktop
flutter run -d windows
```

## License

MIT
