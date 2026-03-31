<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Supply Chain Security Audit — exopack

*Federal-grade audit per EO 14028. 2026-03-30.*

## Summary

| Check | Result | Details |
|-------|--------|---------|
| cargo audit (CVEs) | **1 VULN** | idna 0.3.0 — Punycode label issue |
| Unmaintained crates | **3 WARN** | fxhash, paste, rustls-pemfile |
| Cargo.lock committed | PASS | Pinned, deterministic |
| Duplicate deps | PASS | No duplicates found |
| Outdated direct deps | PASS | All direct deps at latest |
| Large files in git | PASS | None >1MB (removed 277KB unused font) |
| Junk files | PASS | None found |
| Typosquatting | PASS | All dep names match well-known crates |
| Our code: unsafe | PASS | Zero unsafe blocks |
| Our code: command injection | PASS | Only shells out to `cargo` (hardcoded) |

## Vulnerability: RUSTSEC-2024-0421 (idna 0.3.0)

**Severity:** Low for exopack
**Advisory:** `idna` accepts Punycode labels that don't produce non-ASCII when decoded
**Path:** `exopack → reqwest 0.11.27 → cookie_store 0.20.0 → idna 0.3.0`
**Fix available:** idna >= 1.0.0

**Risk assessment:** exopack only makes HTTP requests to `127.0.0.1` (localhost test servers). No user-controlled URLs. No international domain names processed. The vulnerability is not exploitable in exopack's use case.

**Resolution:** Transitive dep via reqwest 0.11. Upgrading reqwest to 0.12+ would fix this but introduces breaking API changes across screenshot, interface, and baked_demo modules. Not worth the churn for a non-exploitable vuln on localhost connections. Documented and accepted.

## Unmaintained Crates

| Crate | Advisory | Path | Risk |
|-------|----------|------|------|
| fxhash 0.2.1 | RUSTSEC-2025-0057 | scraper → selectors → fxhash | Low — hash map impl, no security impact |
| paste 1.0.15 | RUSTSEC-2024-0436 | image → ravif → rav1e → paste | Low — proc macro for token pasting, compile-time only |
| rustls-pemfile 1.0.4 | RUSTSEC-2025-0134 | reqwest 0.11 → rustls-pemfile | Low — PEM parsing, only used for TLS cert loading |

All three are transitive deps behind optional feature gates. None are in the default (triple_sims) build path.

## Outdated Dependencies

All 17 direct dependencies are at their latest versions. `cargo outdated --root-deps-only` reports zero updates needed.

Note: reqwest 0.11.27 is the latest 0.11.x. reqwest 0.12+ exists but is a semver-breaking upgrade.

## Deep Code Review

### Our Source (exopack)
- **unsafe blocks:** 0
- **Command spawning:** `triple_sims.rs` and `baked_demo.rs` spawn `cargo build`/`cargo run` — binary name hardcoded to "cargo", not user-controlled
- **File path construction:** `demo.rs` and `video.rs` sanitize filenames with `.replace(['/', '\\', ':'], "_")` before writes
- **Network:** All HTTP in screenshot/interface/baked_demo targets localhost (127.0.0.1)
- **Env vars read:** `KOVA_DEMO_DIR` (demo.rs), `TEST_DEMO` (triple_sims.rs), `CARGO_PKG_VERSION`/`CARGO_PKG_NAME` (binary) — all expected

### Critical Dependencies — unsafe Usage

| Crate | unsafe count | Assessment |
|-------|-------------|------------|
| tokio 1.50.0 | 1,012 | Expected — async runtime managing OS primitives. Widely audited. |
| serde_json 1.0.149 | 12 | Low — performance-critical JSON parsing paths. Well-audited. |
| reqwest 0.11.27 | 9 | Low — TLS/socket handling. Uses rustls (memory-safe TLS). |
| image 0.25.10 | 6 | Low — pixel buffer manipulation. Well-audited. |
| serde 1.0.228 | 2 | Minimal — core serialization trait impls. |
| wiremock 0.6.5 | 0 | Clean |
| chromiumoxide 0.9.1 | 0 | Clean |
| scraper 0.19.1 | 0 | Clean |

### Unexpected Behavior Checks

| Check | Result |
|-------|--------|
| Deps phoning home (telemetry) | None detected |
| Deps reading unexpected env vars | None detected |
| Deps writing to unexpected paths | None detected — chromiumoxide writes to ~/.cache/chromiumoxide (documented) |
| Deps with excessive network access | wiremock binds localhost (expected — it's a mock server) |

## File Sprawl Cleanup (performed during audit)

| Action | Details |
|--------|---------|
| Removed `fonts/Nunito-Regular.ttf` | 277 KB unused font — referenced in initial commit but never imported by code |
| Removed empty `examples/` dir | Created at init, never populated |
| Untracked `release/` binaries from git | Were committed before .gitignore entry; now only in GitHub Releases |
| Updated `.gitignore` | Added .DS_Store, *.swp, *.orig, *.bak, *.log, *.jks, vendor/ |

## Recommended Actions

1. **Accept** idna 0.3.0 risk — non-exploitable on localhost connections
2. **Monitor** fxhash, paste, rustls-pemfile for replacements in upstream crates
3. **Consider** reqwest 0.12 upgrade when a breaking-change window opens
4. **Run** `cargo audit` before every release
