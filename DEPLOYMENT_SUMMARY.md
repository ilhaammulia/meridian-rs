# Deployment Setup Complete

Meridian RS now fully configurable for Komodo container deployment with flexible branch selection.

## Files Created

| File | Purpose | Size |
|------|---------|------|
| **build.sh** | Main build orchestrator | 8.2K |
| **Dockerfile** | Multi-stage container build | 3.6K |
| **docker-compose.yml** | Multi-service orchestration | 2.5K |
| **komodo-compose.yml** | Komodo-native config | 2.9K |
| **.dockerignore** | Build context optimization | 397B |
| **KOMODO_DEPLOYMENT.md** | Full deployment guide | 7.3K |
| **BUILD_QUICK_REFERENCE.md** | Quick start reference | 4.2K |

## Key Features

### Flexible Build Configurations

**Three build modes:**
1. **Core-only** (minimal): CLI + Rust web server
   ```bash
   ./build.sh
   ```

2. **With web UI** (full): Integrates Next.js dashboard
   ```bash
   ./build.sh --web-enabled true
   ```

3. **Docker** (containerized): Multi-stage build for isolation
   ```bash
   ./build.sh --docker --docker-tag meridian:web --web-enabled true
   ```

### Branch Selection

Mix branches for custom configurations:
```bash
./build.sh --core-branch master --web-branch web-ui --web-enabled true
./build.sh --core-branch vps --web-branch custom-web --docker
```

### Multi-Branch Architecture

- **master**: Core Rust backend (default)
- **web-ui**: Next.js frontend dashboard
- **vps**: Alternative backend implementation

Build script clones, builds, and merges artifacts automatically.

## Quick Deployment Paths

### Local Development
```bash
./build.sh
MERIDIAN_DATA_DIR=~/.meridian ./target/release/meridian-rs
```

### Docker (Single Container)
```bash
./build.sh --docker --docker-tag meridian:web --web-enabled true
docker run -p 3000:3000 -v ~/meridian-data:/data meridian:web
```

### Docker Compose (Multi-Variant)
```bash
docker-compose up -d
# Runs both core and web variants on different ports
```

### Komodo Native
```bash
./build.sh --docker --docker-tag meridian:homelab --web-enabled true
komodo up -f komodo-compose.yml
```

## Configuration

All builds expect:

**Environment variables** (`.env`):
```bash
WALLET_PRIVATE_KEY=<base58-keypair>
MERIDIAN_WALLET=<wallet-address>
RPC_URL=<solana-rpc>
OPENROUTER_API_KEY=<api-key>
DRY_RUN=true  # Always start with true
```

**Strategy config** (`user-config.json`):
```json
{
  "screening": { "timeframe": 3600, "threshold": 0.75 },
  "dryRun": true
}
```

Examples provided:
- `.env.example`
- `user-config.example.json`

Container initialization auto-creates from templates on first run.

## Architecture Overview

```
build.sh (orchestrator)
├── Native builds
│   ├── Core-only: cargo build --release
│   └── With web: merge web-ui assets + cargo build
├── Docker builds
│   ├── Stage 1: Node.js builder (web UI if enabled)
│   ├── Stage 2: Rust builder (core backend)
│   └── Stage 3: Alpine runtime (minimal final image)
└── Branch selection
    ├── Clone core from --core-branch
    ├── Clone web-ui from --web-branch
    └── Merge artifacts pre-build
```

Docker images use multi-stage to minimize:
- **Core**: ~150MB (Rust binary + alpine)
- **With Web**: ~280MB (Rust + Next.js build + alpine)

## Usage Reference

### Common Commands

```bash
# View options
./build.sh --help

# Build just core
./build.sh

# Build with Next.js web UI
./build.sh --web-enabled true

# Build Docker image (core)
./build.sh --docker --docker-tag meridian:core

# Build Docker image (with web)
./build.sh --docker --docker-tag meridian:web --web-enabled true

# Build from custom branches
./build.sh --core-branch vps --web-branch custom-web --web-enabled true --docker

# Run Docker container
docker run -it \
  -p 3000:3000 \
  -v ~/meridian-data:/data \
  -e DRY_RUN=true \
  -e WALLET_PRIVATE_KEY="..." \
  -e MERIDIAN_WALLET="..." \
  -e RPC_URL="..." \
  -e OPENROUTER_API_KEY="..." \
  meridian:web
```

### Deployment Checklist

- [ ] **Prepare config**: Copy `.env.example` → `.env`, fill in secrets
- [ ] **Prepare volume**: Create `/path/to/meridian-data/`
- [ ] **Build image**: `./build.sh --docker --docker-tag meridian:web --web-enabled true`
- [ ] **Test dry-run**: Ensure `DRY_RUN=true` in `.env`
- [ ] **Start container**: `docker run -v /path/to/meridian-data:/data ...`
- [ ] **Verify health**: `curl http://localhost:8080/health`
- [ ] **Access web UI**: `http://localhost:3000` (if web-enabled)
- [ ] **Test commands**: `curl http://localhost:3000/api/status` or REPL commands
- [ ] **Enable live mode**: Change `DRY_RUN=false` when confident

## Documentation

- **[BUILD_QUICK_REFERENCE.md](BUILD_QUICK_REFERENCE.md)** — Quick start, one-liners, common issues
- **[KOMODO_DEPLOYMENT.md](KOMODO_DEPLOYMENT.md)** — Full guide, all options, advanced setup
- **[README.md](README.md)** — Project overview, feature reference
- **[docs/production-operations.md](docs/production-operations.md)** — Production best practices

## Next Steps

1. **Read**: [BUILD_QUICK_REFERENCE.md](BUILD_QUICK_REFERENCE.md) (5 min)
2. **Setup**: Copy `.env.example` and `user-config.example.json`, fill in config
3. **Build**: `./build.sh --docker --docker-tag meridian:web --web-enabled true`
4. **Deploy**: Use docker-compose or Komodo YAML
5. **Monitor**: Check health endpoint and logs

## Branch Details

### master (default core)
- Latest stable Rust backend
- Includes Axum-based local web server
- Used for native binaries and Docker core image

### web-ui (optional frontend)
- Modern Next.js dashboard
- Real-time position tracking
- Config editor, cycle logs, decision history
- Integrated with master backend when `--web-enabled true`

### vps (alternative backend)
- Different backend implementation
- Can replace master for custom deployments
- Specified via `--core-branch vps`

## Support

For issues:
1. Check [BUILD_QUICK_REFERENCE.md#Common Issues](BUILD_QUICK_REFERENCE.md)
2. Verify prerequisites: `cargo --version`, `docker --version`, `git --version`
3. Test single component: `./build.sh --help`, `docker version`, etc.
4. Review configuration: `.env`, `user-config.json`
5. Check container logs: `docker logs <container>` or `docker-compose logs`
