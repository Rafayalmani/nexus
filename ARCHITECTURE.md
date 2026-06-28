# Nexus P2P Architecture

## Overview

Nexus is a decentralized workspace synchronization platform built on P2P technology. User devices collectively form the cloud, eliminating the need for third-party storage.

## Core Components

### 1. Coordinator (Go)

**Responsibilities:**
- Device registry and metadata storage
- Online/offline status management
- WebSocket presence updates
- Pending sync queue (metadata only)

**Database Schema:**
```sql
- accounts (email, created_at)
- devices (device_id, public_key, last_ip, last_port, is_online)
- sync_queue (device_id, file_path, operation)
```

**Never stores:**
- File contents
- Chunk data
- Clipboard data
- User-created data

### 2. Sync Engine (Rust)

**Features:**
- Filesystem watching (notify crate)
- File chunking (4MB default)
- Delta detection (fast_rsync)
- SHA-256 chunk hashing
- Smart ignore rules

**Key Modules:**
- `watcher`: Monitors filesystem changes
- `chunker`: Breaks files into chunks
- `hasher`: Computes SHA-256 hashes
- `ignore`: Smart ignore rules (node_modules, .git, etc.)

### 3. Crypto Layer (Rust)

**Algorithms:**
- X25519 for key exchange
- ChaCha20-Poly1305 for AEAD encryption
- Ed25519 for device signatures

**Flow:**
1. Devices generate X25519 keypairs
2. Exchange public keys via coordinator
3. Perform ECDH to derive shared secret
4. Use shared secret for ChaCha20-Poly1305
5. Sign all messages with Ed25519

### 4. Networking (Rust)

**P2P Connection Flow:**
1. STUN discovery (get public IP:port)
2. UDP hole punching for NAT traversal
3. QUIC transport for reliable P2P
4. Fall back to relay server if direct connection fails

**Relay Server:**
- Temporarily stores encrypted chunks
- TTL: 24 hours
- Auto-deletes after delivery
- Never decrypts data

### 5. Desktop App (Flutter)

**Screens:**
- Login/Account
- Device pairing (QR code + 6-digit)
- Folder permissions
- Device dashboard
- Sync status

**Integration:**
- Calls Rust engine via flutter_rust_bridge
- Local SQLite for version history
- WebSocket to coordinator for presence

### 6. Mobile App (Flutter)

**Features:**
- Photo/video browsing and download
- File browser across devices
- Clipboard sync
- Device status

## Data Flow

### File Sync

```
Desktop File Edit
    ↓
Filesystem Watcher detects change
    ↓
Sync Engine computes chunks & hashes
    ↓
Crypto signs & encrypts chunks
    ↓
Query Coordinator for Laptop online status
    ↓
P2P Connection (direct if possible, relay if not)
    ↓
Laptop receives encrypted chunks
    ↓
Crypto decrypts & verifies signature
    ↓
Sync Engine reconstructs file
    ↓
Filesystem watch triggers on Laptop
```

### Device Pairing

```
New Device Scan QR Code
    ↓
Receive Device ID + Initial Public Key
    ↓
Exchange full X25519 public keys
    ↓
Perform ECDH key exchange
    ↓
Derived shared secret used for all future encryption
    ↓
Device appears in dashboard
```

## Offline Mode

- Changes queued locally
- Coordinator tracks pending changes
- On reconnect, Sync Engine compares file hashes
- Delta detected and synced
- Conflicts resolved by last-write-wins

## Security

- **Transport:** All data encrypted with ChaCha20-Poly1305
- **Identity:** Devices identified by Ed25519 keys
- **Coordinator:** Zero-knowledge (never sees plaintext)
- **Relay:** Cannot decrypt, only routes packets
- **Local:** Device-local version history SQLite

## Performance

- **Delta Sync:** Only changed chunks transmitted
- **Compression:** Chunks are pre-compressed before encryption
- **Streaming:** Large files streamed on-demand
- **LAN Optimization:** Direct local network transfers without internet

## Scalability

- Each device stores only its files
- No central data repository
- Coordinator is stateless (can be scaled horizontally)
- P2P connections reduce bandwidth bottleneck
