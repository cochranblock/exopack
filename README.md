# exopack

Testing augmentation for Rust binaries: screenshot capture, video recording, interface creation, API mocking, TRIPLE SIMS.

Used by cochranblock, kova, approuter, oakilydokily, whyyoulying, wowasticker for test binaries (`*-test`).

## Features

- **screenshot** — Pure Rust HTML→SVG→PNG capture (no Chrome)
- **interface** — Test server harness, HTTP client helpers
- **triple_sims** — Run test runner 3 times; all must pass
- **devtools** — Headless browser console check via CDP
- **mock** — WireMock for on-demand API mocking
- **video** — xcap screen capture + recording

## Docs

- [docs/testing_architecture.md](docs/testing_architecture.md) — Two-binary test model
- [docs/ROUGH_DRAFT_EXOPACK.md](docs/ROUGH_DRAFT_EXOPACK.md) — Design notes
