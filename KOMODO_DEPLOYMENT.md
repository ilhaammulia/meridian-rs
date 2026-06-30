# Meridian RS - Komodo Container Deployment Guide

Guide for deploying Meridian RS into homelab Komodo container environment.

## Build System Overview

Meridian RS supports three build configurations:

| Config | Core | Web UI | Use Case |
|--------|------|--------|----------|
| **core-only** (default) | master branch | None | Minimal footprint, CLI only |
| **with-web** | master branch | web-ui branch | Full control surface, local dashboard |
| **docker** | Containerized | Containerized | Komodo deployment, isolation |

## Quick Start

### 1. Build Core-Only Binary

Minimal build (~80MB), CLI + local Rust web server:

```bash
./build.sh
```

Output: `./target/release/meridian-rs`

### 2. Build with Integrated Web UI

Combines master (backend) + web-ui branch (Next.js frontend):

```bash
./build.sh --web-enabled true
```

**Notes:**
- Requires `npm` or `pnpm` installed locally
- Next.js build happens before Rust compilation
- Resulting binary: ~120MB
- Web UI served on port 3000 (same as core)

### 3. Build Docker Image

For Komodo container environment:

```bash
# Core-only image
./build.sh --docker --docker-tag meridian:core

# With integrated web UI
./build.sh --docker --docker-tag meridian:web --web-enabled true

# Custom branches
./build.sh --docker \
  --core-branch master \
  --web-branch web-ui \
  --docker-tag meridian:custom
```

Images will be: `meridian:core` (~150MB) or `meridian:web` (~280MB)

## Build Script Options

```bash
./build.sh [OPTIONS]

Options:
  --core-branch BRANCH      Source branch for backend (default: master)
  --web-branch BRANCH       Source branch for web UI (default: web-ui)
  --web-enabled true|false  Include web UI build (default: false)
  --output DIR              Native binary output dir (default: ./target/release)
  --docker                  Build Docker image instead of binary
  --docker-tag TAG          Docker image name:tag (default: meridian:latest)
  --help                    Show help
```

## Docker Deployment

### Option A: Single Service (Recommended for Homelab)

```bash
# Build image
./build.sh --docker --docker-tag meridian:latest

# Run with minimal config
docker run -it --rm \
  -p 3000:3000 \
  -p 8080:8080 \
  -v ~/meridian-data:/data \
  -e DRY_RUN=true \
  -e RPC_URL=https://api.mainnet-beta.solana.com \
  meridian:latest
```

### Option B: Docker Compose (Multi-Config)

Deploy both core and web-ui variants:

```bash
# Copy environment template
cp .env.example .env

# Edit configuration
$EDITOR .env

# Start services
docker-compose up -d

# View logs
docker-compose logs -f meridian-core
docker-compose logs -f meridian-web
```

Services:
- `meridian-core` → port 3000, 8080
- `meridian-web` → port 3001, 8081
- Shared volume: `meridian-data`

### Option C: Komodo Native

For Komodo container orchestration:

```yaml
# komodo/meridian.yaml
version: '3'
services:
  meridian-rs:
    image: meridian:web
    container:
      restart: unless-stopped
      health_check:
        test: curl http://localhost:8080/health
        interval: 30s
        timeout: 10s
    ports:
      - "3000:3000/tcp"
      - "8080:8080/tcp"
    volumes:
      - meridian-data:/data
    environment:
      DRY_RUN: "true"
      MERIDIAN_DATA_DIR: "/data"
      MERIDIAN_LOG_STYLE: "pretty"
      MERIDIAN_WEB_ADDR: "0.0.0.0:3000"
      HEALTH_PORT: "8080"
      WALLET_PRIVATE_KEY: "${WALLET_PRIVATE_KEY}"
      MERIDIAN_WALLET: "${MERIDIAN_WALLET}"
      RPC_URL: "https://api.mainnet-beta.solana.com"
      OPENROUTER_API_KEY: "${OPENROUTER_API_KEY}"
      LLM_MODEL: "openai/gpt-4o-mini"
```

Deploy: `komodo apply -f meridian.yaml`

## Configuration

### Required Environment Variables

```dotenv
# Wallet
WALLET_PRIVATE_KEY=<base58-encoded-solana-keypair>
MERIDIAN_WALLET=<wallet-address>

# RPC
RPC_URL=https://api.mainnet-beta.solana.com

# LLM (for agent reasoning)
OPENROUTER_API_KEY=<your-openrouter-key>
LLM_MODEL=openai/gpt-4o-mini
```

### Runtime Options

```dotenv
# Safety first - always start with DRY_RUN=true
DRY_RUN=true

# Logging style: pretty | plain
MERIDIAN_LOG_STYLE=pretty

# Web UI binding (container must use 0.0.0.0 for external access)
MERIDIAN_WEB_ADDR=0.0.0.0:3000

# Data directory (use /data in containers)
MERIDIAN_DATA_DIR=/data
```

### Config Files (Mounted at Runtime)

Inside container, create these in `/data/`:

1. **`.env`** - Runtime secrets and API keys
   ```bash
   cp .env.example /path/to/meridian-data/.env
   # Edit with your wallet, RPC, LLM credentials
   ```

2. **`user-config.json`** - Strategy and screening parameters
   ```bash
   cp user-config.example.json /path/to/meridian-data/user-config.json
   # Customize screening thresholds, strategies, etc.
   ```

## Health Check

Both images include health endpoints:

```bash
# Status / health
curl http://localhost:8080/health

# Web UI (if enabled)
curl http://localhost:3000

# Interactive REPL (when running as CLI)
# Just run: meridian-rs
# Commands: status, screen, manage, quit
```

## Volume Structure

Container `/data` directory persists:

```
/data/
├── .env                      # Runtime secrets (created on first run)
├── user-config.json          # Strategy config (created on first run)
├── meridian-state.json       # Active positions
├── pool-memory.json          # Pool notes and history
├── lessons.json              # Agent learnings and performance
└── discord-signals.json      # Signal queue (if enabled)
```

Mount to homelab persistent storage:

```bash
# Docker
-v /storage/meridian-data:/data

# Docker Compose
volumes:
  meridian-data:
    driver: local
    driver_opts:
      type: none
      o: bind
      device: /storage/meridian-data

# Komodo
volumes:
  meridian-data:
    path: /storage/meridian-data
```

## Build Troubleshooting

### "cargo not found"
Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### "npm/pnpm not found" (when building with web-enabled)
Web UI build skipped. Either:
- Install Node: `brew install node` (macOS) or from nodejs.org
- Build core-only: `./build.sh` (no web UI)
- Use Docker: `./build.sh --docker --docker-tag meridian:web --web-enabled true`

### Docker build fails with "failed to download"
Check internet connection and git access:
```bash
git ls-remote https://github.com/FlipZ3ro/meridian-rs.git
```

### Binary or image very large
Normal sizes:
- Core binary: ~80MB (release build)
- Core Docker: ~150MB (multi-stage, alpine base)
- With Web: ~120MB binary, ~280MB Docker image

Remove symbols to shrink (Dockerfile):
```dockerfile
RUN strip /app/meridian-rs  # Reduces ~30MB
```

## Next Steps

1. **Initial dry-run**: Start with `DRY_RUN=true` in `.env`
2. **Configure wallet**: Add `WALLET_PRIVATE_KEY` and `MERIDIAN_WALLET` to `.env`
3. **Set strategy**: Customize `user-config.json` screening thresholds
4. **Test screening**: Run one-shot: `meridian-rs screen --limit 3`
5. **Enable live mode**: Change `DRY_RUN=false` when confident

For production:
- Use encrypted env (see: [production-operations.md](docs/production-operations.md))
- Mount state to persistent storage (NFS, etc.)
- Enable monitoring with health endpoint
- Set up restart policies and logging

## References

- Full docs: [docs/production-operations.md](docs/production-operations.md)
- Config reference: `.env.example`, `user-config.example.json`
- Agent mode: Interactive REPL with `screen`, `manage`, `quit` commands
