# Meridian RS

**Meridian RS** is a Rust rewrite of the autonomous DLMM liquidity provider agent for Meteora on Solana.

## Features

- Config system with LLM integration
- Solana wallet connection
- DLMM position management (deploy/close)
- Screening engine
- ReAct-style agent loop with LLM
- Management & Screening cycles
- Web UI (Terminal + Cycle Log)

## Current Status

This project is still in early development. Core modules are implemented, but full automation (TP/SL, real cycles) is still being built.

## Project Structure

```
src/
├── main.rs
├── cycle.rs          # Management & Screening cycles
├── config/           # Config loader + types
├── tools/            # DLMM, Wallet, Screening, Executor
├── agent/            # ReAct Agent Loop
├── llm.rs            # OpenAI-compatible LLM client
├── state/            # Position tracking
├── web.rs            # Web UI (Terminal + Cycle Log)
└── utils/
```

## Run

```bash
cargo run
```

Web UI will be available at `http://localhost:3000`

## Config

Copy `user-config.example.json` to `user-config.json` and modify as needed.

**Never commit your real `user-config.json` or API keys.**

## License

MIT

---

**Note**: This is a work in progress. Many features are still being implemented to match the original Node.js version.