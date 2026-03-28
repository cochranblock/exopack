<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Security Posture — exopack

*2026-03-27.*

## Cryptography

exopack does **not** implement or use any cryptographic primitives directly. It is a testing library, not a security-sensitive application.

- **TLS:** reqwest uses `rustls` (Rust-native TLS) for HTTP requests in the screenshot and interface modules. No OpenSSL dependency.
- **No secrets management:** exopack does not store, encrypt, or transmit secrets. It runs test binaries and compares screenshots.
- **No authentication:** No user accounts, tokens, or session management.

## Input Validation

| Input | Validation | Location |
|-------|-----------|----------|
| CLI project_dir | Checks `Cargo.toml` exists before proceeding | `src/bin/exopack.rs` |
| CLI bin_name | Parsed from Cargo.toml `[[bin]]` sections, validated against `-test` suffix | `src/triple_sims.rs:f63_discover_test_bin` |
| CLI subcommand | Explicit match against known commands, rejects unknown | `src/bin/exopack.rs` |
| Screenshot URLs | Built from base_url + path concatenation (no user-controlled injection point) | `src/screenshot.rs` |
| File paths | Sanitized: `name.replace(['/', '\\', ':'], "_")` before filesystem writes | `src/demo.rs`, `src/video.rs` |
| Image files | Parsed by `image` crate (well-audited, handles malformed PNGs safely) | `src/screenshot.rs:f71` |

## Attack Surface

| Surface | Risk | Mitigation |
|---------|------|-----------|
| CLI args | Low | Only `live-demo` subcommand accepted. Args passed to cargo (trusted toolchain). |
| Network (screenshot) | Low | HTTP requests to localhost test servers only. 10s timeout. |
| Network (devtools) | Medium | Downloads Chromium binary on first use. Fetched from official distribution via chromiumoxide_fetcher. |
| Filesystem (screenshots) | Low | Writes to `~/.cache/screenshots/` only. Creates dirs with default permissions. |
| Filesystem (demos) | Low | Writes JSON to `~/.kova/demos/`. Filename sanitization prevents path traversal. |
| Process spawning | Medium | `live-demo` spawns `cargo build` + `cargo run` as child processes. Args come from CLI (user-controlled, trusted context). |

## No `unsafe` Code

exopack source contains **zero `unsafe` blocks**. Dependencies (image, tokio, etc.) may use unsafe internally but are widely audited.

## Error Handling

All errors use `Result<T, String>` or `std::io::Result`. No `unwrap()` in library code paths (only in tests). Panic-on-abort in release builds ensures no stack unwinding.
