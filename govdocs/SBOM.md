<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Software Bill of Materials (SBOM) — exopack

*Per Executive Order 14028, Section 4(e). Generated 2026-03-27.*

## Direct Dependencies

All deps from crates.io. Pinned via `Cargo.lock` (committed).

### Default features (no deps — zero transitive)

The exopack binary with only `triple_sims` has **zero external dependencies**. It uses only the Rust standard library (`std::process::Command`, `std::time`, `std::fs`).

### All features enabled (17 direct deps)

| Crate | Version | License | Feature gate | Purpose |
|-------|---------|---------|-------------|---------|
| ab_glyph | 0.2.32 | Apache-2.0 | screenshot | Font glyph rasterization |
| base64 | 0.21.7 | MIT/Apache-2.0 | screenshot | Base64 encoding for embedded assets |
| chromiumoxide | 0.9.1 | MIT/Apache-2.0 | devtools | Headless Chromium via CDP |
| dirs | 5.0.1 | MIT/Apache-2.0 | screenshot,video,devtools | Platform cache/home dir paths |
| fontdb | 0.23.0 | MIT | screenshot | System font database |
| futures | 0.3.32 | MIT/Apache-2.0 | devtools | Async stream utilities |
| image | 0.25.10 | MIT/Apache-2.0 | screenshot,video | PNG read/write, pixel manipulation |
| reqwest | 0.11.27 | MIT/Apache-2.0 | screenshot,interface | HTTP client (rustls TLS) |
| resvg | 0.47.0 | MPL-2.0 | screenshot | SVG rendering to raster |
| scraper | 0.19.1 | MIT | screenshot | HTML parsing (CSS selectors) |
| serde | 1.0.228 | MIT/Apache-2.0 | video,demo | Serialization framework |
| serde_json | 1.0.149 | MIT/Apache-2.0 | mock,demo,devtools,baked_demo | JSON serialization |
| tiny-skia | 0.12.0 | BSD-3-Clause | screenshot | 2D rendering (rasterization) |
| tokio | 1.50.0 | MIT | screenshot,interface,devtools | Async runtime |
| usvg | 0.47.0 | MPL-2.0 | screenshot | SVG parsing and simplification |
| wiremock | 0.6.5 | MIT/Apache-2.0 | mock | HTTP mock server |
| xcap | 0.9.3 | Apache-2.0 | video | Screen capture (platform-native) |

### TLS Stack

reqwest uses `rustls` (not OpenSSL). No C dependencies for TLS.
chromiumoxide uses `rustls` for fetcher downloads.

### Build Dependencies

None. No proc-macro crates beyond serde_derive (pulled transitively by serde).

## Verification

```bash
# Regenerate this list:
cargo tree --depth 1 -p exopack --all-features

# Verify Cargo.lock matches:
cargo verify-project

# Audit for known vulnerabilities:
cargo audit
```

## License Compatibility

All deps are MIT, Apache-2.0, BSD-3-Clause, or MPL-2.0. exopack itself is under the Unlicense (public domain). No GPL dependencies. No copyleft contamination.
