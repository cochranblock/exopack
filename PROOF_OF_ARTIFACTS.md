<!-- Unlicense — cochranblock.org -->

# Proof of Artifacts

*Concrete evidence that this project works, ships, and is real.*

> The quality gate behind every CochranBlock binary. No external test frameworks — the test binary IS the CI.

## Architecture

```mermaid
flowchart TD
    App[App Binary] --> Core[Shared Library]
    Test[Test Binary *-test] --> Core
    Test --> Exopack[exopack]
    Exopack --> TripleSIMS[TRIPLE SIMS: 3-pass determinism]
    Exopack --> Screenshot[Sim 4: Visual Regression]
    Exopack --> DevTools[DevTools: Headless Chromium CDP]
    Exopack --> Mock[Mock: WireMock API stubs]
    Exopack --> Interface[Interface: Random port + HTTP client]
    Exopack --> Video[Video: xcap screen recording]
    Exopack --> Demo[Demo: Record/replay user actions]
    Exopack --> BakedDemo[BakedDemo: Zero-input automation]
```

## Build Output

| Metric | Value |
|--------|-------|
| Modules | 8 (triple_sims, screenshot, devtools, mock, interface, video, demo, baked_demo) |
| Feature gates | 7 feature-gated + video (always compiled, xcap optional) |
| Projects using exopack | 5+ (cochranblock, kova, oakilydokily, whyyoulying, wowasticker) |
| Architecture doc | 2,286 lines — testing philosophy, patterns, anti-patterns |
| TRIPLE SIMS passes | 3 sequential runs, all must pass (eliminates flaky tests) |
| Visual regression | Sim 4: capture → baseline → pixel diff → diff image → per-page report |
| Screenshot method | Pure Rust HTML→SVG→PNG (no Chrome dependency for basic capture) |
| Unit tests | 17 across screenshot, triple_sims, demo, video |

## Key Artifacts

| Artifact | Description |
|----------|-------------|
| TRIPLE SIMS | Run test suite 3x sequentially — all must pass. Detects race conditions, non-determinism, flaky tests |
| Two-Binary Model | Production binary has zero test deps. Test binary is self-contained quality gate |
| Sim 4 Visual Regression | Full orchestrator: capture → auto-baseline → pixel diff (configurable tolerance/threshold) → red-highlight diff PNG → per-page pass/fail report. `f76_update_baselines` to accept new state |
| Mock Server | WireMock integration on random ports — isolated integration tests without real APIs |
| Demo Record/Replay | Capture WebClick, WebInput, ApiCall, EguiSend actions as JSON for automated replay |
| Baked Demo | Zero-user-input automation: CLI subcommands + all HTTP endpoints exercised |
| HTTP Harness | Bind to :0 (random port) + cookie-store client — test servers without port conflicts |

## How to Verify

```bash
# Any project using exopack:
cargo run -p cochranblock --bin cochranblock-test --features tests
# Runs: clippy → TRIPLE SIMS (3 passes) → exit 0 or 1

# exopack standalone (requires triple_sims feature for the binary):
cargo run -p exopack --features triple_sims -- live-demo <project_dir>
```

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
