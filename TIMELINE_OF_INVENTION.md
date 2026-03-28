<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

### 2026-03-27 — Federal Compliance Documentation

**What:** 11 federal compliance documents in `govdocs/`: SBOM (EO 14028), SSDF (NIST SP 800-218), supply chain integrity, security posture, accessibility (Section 508), privacy impact assessment, FIPS 140-2/3 status, FedRAMP notes, CMMC mapping, ITAR/EAR export classification, federal agency use cases.
**Why:** Preparing for federal procurement. Every claim verifiable from source code.
**Commit:** `f0580f7`
**AI Role:** AI authored all 11 documents based on source code analysis. Human directed the compliance scope.

### 2026-03-27 — User Story Analysis + CLI Fixes

**What:** Full 10-point user story walkthrough: discovery, install, happy path, edge cases, feature gaps, competitor check. Scored 5.4/10 (honest). Implemented top 3 fixes: `--help`/`--version` flags, validation order fix (check dir before Cargo.toml parse), context-aware error messages.
**Why:** Product needed a real user perspective. CLI was missing basic expected flags.
**Commits:** `6c31861`
**AI Role:** AI performed user walkthrough and identified issues. AI implemented fixes. Human directed scope.

### 2026-03-27 — P13 Tokenization + Binary Size

**What:** Applied Kova P13 compression mapping: 28 functions → f60–f95, 8 types → t60–t67, 19 fields → s60–s78. Created `docs/compression_map.md`. Added release profile: `opt-level='z'`, `lto=true`, `codegen-units=1`, `panic='abort'`, `strip=true`. Binary: 314 KB (aarch64-apple-darwin).
**Why:** Consistency with kova compression protocol. Smallest possible binary for the test runner.
**Commit:** `ba1ff82`
**AI Role:** AI performed complete symbol inventory and rename. Human specified P13 protocol.

### 2026-03-27 — QA Round 2: Test Race Condition Fix

**What:** PID-scoped temp directories in all unit tests. Tests were failing when two processes ran concurrently (same temp dir names). Now each process uses `{name}_{pid}` pattern.
**Why:** Concurrent `cargo test` runs were producing false failures.
**Commit:** `e76fffb`
**AI Role:** AI diagnosed the race condition from test output and implemented the fix.

### 2026-03-27 — Sim 4 Visual Regression Orchestrator

**What:** Full visual regression pipeline: `f73` captures screenshots, auto-creates baselines on first run, compares against baselines with configurable tolerance/threshold, generates red-highlight diff PNGs for failures, returns structured `t63` report with per-page pass/fail. `f76` accepts new baselines. Directory layout: `~/.cache/screenshots/{os}/{project}/{current,baselines,diffs}/`.
**Why:** Architecture doc described visual regression (Section 3.4) but it was never implemented.
**Commit:** `8724b66`
**AI Role:** AI designed and built the orchestrator. Human specified the Sim 4 concept.

### 2026-03-27 — Visual Regression + POST Mocks + Unit Tests (0→17)

**What:** `f71` pixel-level screenshot comparison with tolerance/threshold. `f72` red-highlight diff image generation. POST mock helpers (`f85`, `f86`). Custom status mock (`f87`). 13 unit tests across screenshot, triple_sims, demo, video modules.
**Why:** Screenshot module could capture but never compare. Mock module only had GET. Zero unit tests.
**Commits:** `5233c5d`
**AI Role:** AI implemented all features and tests. Human specified requirements.

### 2026-03-27 — Documentation Audit (3 passes)

**What:** Three-pass doc review. Pass 1: Added demo/baked_demo to README features and mermaid diagram, expanded feature descriptions, fixed compression ID conflicts (screenshot f61→f70, devtools f62→f74), removed duplicate approuter in ROUGH_DRAFT table. Pass 2: Added BakedDemo to PROOF_OF_ARTIFACTS diagram, fixed feature gate count. Pass 3: Fixed f63 doc prefix, added demo/baked_demo to one-liner, fixed standalone command (needs `--features triple_sims`).
**Why:** Docs were stale — missing 2 modules, conflicting compression IDs, broken CLI command.
**Commits:** `6a356f9`, `0e37662`, `d099d7f`
**AI Role:** AI performed full audit and fixes. Human directed review scope.

### 2026-03-24 — Proof of Artifacts + Timeline of Invention

**What:** Added zero-cloud banner, Proof of Artifacts, Timeline of Invention to exopack.
**Why:** Consistency with CochranBlock documentation standards across all repos.
**Commit:** `571d4d0`
**AI Role:** AI generated documents. Human approved.

### 2026-03-11 — Standalone with Inlined Dependencies

**What:** Made exopack standalone — inlined serde_json/tokio deps, updated README with wire diagram.
**Why:** Projects consuming exopack via git dep shouldn't need to manage transitive dependency conflicts.
**Commits:** `4261765`, `01c55e1`
**AI Role:** AI refactored dependency tree. Human decided on standalone packaging strategy.

### 2026-03-11 — Initial Release: Full Testing Framework

**What:** Complete testing augmentation library in one sprint: TRIPLE SIMS, screenshot (HTML→SVG→PNG), DevTools (headless Chromium), mock (WireMock), interface (random port harness), video (xcap), demo (record/replay), baked_demo (zero-input automation). 2,286-line architecture guide. Embedded Nunito font for screenshot text rendering.
**Why:** Every CochranBlock project needed a consistent quality gate. No external test frameworks — the binary tests itself.
**Commit:** `a4989c1`
**AI Role:** AI generated all 8 modules and architecture documentation. Human designed the two-binary model, TRIPLE SIMS pattern, and feature-gate strategy. Human verified every module works across cochranblock, kova, and oakilydokily.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
