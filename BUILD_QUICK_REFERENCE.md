# Build Quick Reference

Fastest path to deployment. Full docs in [KOMODO_DEPLOYMENT.md](KOMODO_DEPLOYMENT.md).

## One-Liners

### Core-Only (CLI + minimal web server)
```bash
./build.sh
# Output: ./target/release/meridian-rs
```

### Core + Next.js Web UI (full dashboard)
```bash
./build.sh --web-enabled true
# Requires: npm or pnpm
# Output: ./target/release/meridian-rs (includes web assets)
```

### Docker: Core-Only
```bash
./build.sh --docker --docker-tag meridian:core
docker run -it -p 3000:3000 -v ~/meridian-data:/data meridian:core
```

### Docker: With Web UI
```bash
./build.sh --docker --docker-tag meridian:web --web-enabled true
docker run -it -p 3000:3000 -v ~/meridian-data:/data meridian:web
```

### Docker Compose (Both Variants)
```bash
cp .env.example .env
# Edit .env with your config
docker-compose up -d
# Access: http://localhost:3000 (core) or http://localhost:3001 (web)
```

### Komodo Deployment
```bash
./build.sh --docker --docker-tag meridian:homelab --web-enabled true
komodo up -f komodo-compose.yml
```

## Branch Selection

Mix branches for custom builds:

```bash
# Use vps branch for backend + web UI from web-ui
./build.sh --core-branch vps --web-branch web-ui --web-enabled true --docker

# Custom core + web-ui
./build.sh --core-branch custom-feature --web-enabled true
```

## Environment Setup

Every deployment needs config. First run creates templates:

**Docker/Komodo:**
```bash
# Container auto-creates from examples on first run
# Just mount /data and the image will initialize with templates

# Then edit your config:
$EDITOR ~/meridian-data/.env
$EDITOR ~/meridian-data/user-config.json
```

**Native Binary:**
```bash
# Manual setup
mkdir -p ~/.meridian
cp .env.example ~/.meridian/.env
cp user-config.example.json ~/.meridian/user-config.json

# Edit with your secrets
$EDITOR ~/.meridian/.env

# Run
MERIDIAN_DATA_DIR=~/.meridian ./target/release/meridian-rs
```

## Testing the Build

### Verify Binary Works
```bash
./target/release/meridian-rs --help
./target/release/meridian-rs status
```

### Verify Docker Image
```bash
docker images | grep meridian
docker run --rm meridian:core --help
docker run --rm -p 8080:8080 meridian:core status
```

### Health Check
```bash
# If running
curl http://localhost:8080/health

# Expected: {"status":"ok"} or similar
```

## Common Issues

| Issue | Fix |
|-------|-----|
| `cargo not found` | Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| `npm not found` (with `--web-enabled true`) | Install Node or skip web: use `./build.sh` without flag |
| Docker build fails | Check git access: `git ls-remote https://github.com/FlipZ3ro/meridian-rs.git` |
| Binary ~200MB | Normal. Use `--output dir` to organize, or use Docker multi-stage (smaller) |
| Permission denied | `chmod +x ./build.sh` |

## Config Secrets

**Never commit to git:**
- `.env` (wallet keys, API keys)
- `user-config.json` (if contains API keys)
- `.env.local` variants

Templates provided:
- `.env.example` → copy to `.env` and fill in
- `user-config.example.json` → copy to `user-config.json` and customize

**In containers:** Mount from host `~/.meridian/` or persistent volume.

## What Gets Built

| Artifact | Size | Contents |
|----------|------|----------|
| `meridian-rs` binary (core) | ~80MB | Rust binary, Solana + LLM logic |
| `meridian:core` Docker | ~150MB | Binary + runtime base (alpine) |
| `meridian-rs` binary (web) | ~120MB | Rust binary + Next.js build output |
| `meridian:web` Docker | ~280MB | Binary + web + runtime |

Multi-stage Dockerfile compiles in temp stages; final image only contains:
- Rust binary
- Runtime libraries (ca-certs, curl, etc.)
- Web assets (if `WEB_ENABLED=true`)
- Example configs

## Deployment Checklist

- [ ] Config ready: `.env` with wallet, RPC, LLM keys
- [ ] Volume mounted: `-v ~/meridian-data:/data`
- [ ] Test DRY_RUN: `DRY_RUN=true` in `.env`
- [ ] Web UI accessible: `http://localhost:3000`
- [ ] Health check passing: `curl http://localhost:8080/health`
- [ ] Strategy configured: Customize `user-config.json`
- [ ] Ready for live: Change `DRY_RUN=false` when confident

## Next: Full Docs

See [KOMODO_DEPLOYMENT.md](KOMODO_DEPLOYMENT.md) for:
- Detailed build options
- Configuration reference
- Advanced Komodo setup
- Troubleshooting
- Production operations
