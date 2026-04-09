<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why. Proves human-piloted AI development — not generated spaghetti.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

## How to Read This Document

Each entry follows this format:

- **Date**: When the work shipped (not when it was started)
- **What**: Concrete deliverable — binary, feature, fix, architecture change
- **Why**: Business or technical reason driving the decision
- **Commit**: Short hash(es) for traceability
- **AI Role**: What the AI did vs. what the human directed

This document exists because AI-assisted code has a trust problem. Anyone can generate 10,000 lines of spaghetti. This timeline proves that a human pilot directed every decision, verified every output, and shipped working software.

---

## Human Revelations — Invented Techniques

*Novel ideas that came from human insight, not AI suggestion. These are original contributions to the field.*

### Triple Sims — Run 3x, All Must Pass (March 2026)

**Invention:** Run the entire test suite 3 times in sequence. All 3 runs must pass for the gate to be green. If any single run fails, the gate is red. Catches flaky tests, race conditions, and non-deterministic behavior that single-run CI misses.

**The Problem:** CI pipelines run tests once. A test that passes 90% of the time looks green. Race conditions in temp file handling, port binding, or async timing cause intermittent failures that developers dismiss as "flaky." The test suite lies about reliability — it reports green when the code is broken.

**The Insight:** Military simulation ranges run the same scenario 3 times before certifying a system. If the weapon hits the target once, it might be luck. Three hits is a pattern. Three misses is a failure. Apply the same standard to software: run it 3 times, all 3 must pass, no exceptions.

**The Technique:**
1. `triple_sims::f60`: takes a test function, runs it 3 times sequentially
2. Each run gets a fresh environment (new temp dirs, new ports)
3. All 3 must return Ok — any failure fails the gate
4. Output reports per-run pass/fail with timing
5. Used as the CI gate for every CochranBlock project via the test binary

**Result:** Caught race conditions in 4 projects (exopack temp dir collisions, ronin-sites port conflicts, oakilydokily async timing). Tests that passed on single runs failed on triple. The gate is honest.

**Named:** Triple Sims (TRIPLE SIMS)
**Commit:** `a4989c1` (initial release)
**Origin:** Military weapons qualification — "three rounds, three hits." Michael Cochran's experience with live-fire simulation ranges where systems must demonstrate repeatable performance, not one-time luck.

### Test Binary Augmentation Pattern (March 2026)

**Invention:** The same Rust binary serves as both the production application and the test runner. No separate test framework, no test harness, no pytest/jest/mocha. The binary tests itself with a `--test` flag or a separate `{project}-test` binary that imports the library and runs it through Triple Sims.

**The Problem:** Test frameworks are separate programs that import your code and probe it from the outside. They have their own dependencies, their own bugs, their own versioning. A test framework that doesn't match the production runtime can produce false positives. And test framework dependencies bloat the supply chain.

**The Insight:** The binary already contains all the code. It already has all the types, all the routes, all the handlers. Why import it into a separate framework? Let the binary test itself. The test binary is the same crate with a `tests` feature gate — same dependencies, same compiler, same runtime. The test IS the product.

**The Technique:**
1. Each project has `src/tests.rs` with smoke tests (health checks, route verification, API calls)
2. A `{project}-test` binary in `src/bin/` spawns the server on a random port, runs tests, cleans up
3. Tests run through `exopack::triple_sims::f60` — 3x execution, all must pass
4. The test binary shares the exact same library code as the production binary
5. `cargo run --bin {project}-test --features tests` is the CI command

**Result:** Zero external test frameworks across 16 repositories. The test binary IS the CI pipeline. Binary supply chain is smaller (no test framework deps in production). Tests run against the exact same code that ships.

**Named:** Two-Binary Model (prod binary + test binary, same crate)
**Commit:** `a4989c1` (initial release)
**Origin:** Embedded systems testing — on a microcontroller, you can't install pytest. The firmware tests itself. Applied to web applications: the web server tests itself.

### Visual Regression Orchestrator (March 2026)

**Invention:** A screenshot-based visual regression system where the first run auto-creates baselines, subsequent runs compare against baselines with configurable tolerance, and failures produce red-highlight diff PNGs showing exactly which pixels changed.

**The Problem:** CSS changes break layouts in ways that unit tests can't catch. A button moves 2px left. A font size changes. The page "works" (all routes return 200) but looks wrong. Visual regression tools exist but they're JavaScript-based, require Node.js, and add heavyweight dependencies.

**The Insight:** A Rust binary can capture screenshots (HTML-to-SVG-to-PNG) without a browser. Compare pixel-by-pixel with tolerance for anti-aliasing. Generate diff images that highlight changes in red. Store baselines in a known directory. The first run IS the baseline — no manual setup.

**The Technique:**
1. `f73`: capture screenshots of each page, store in `~/.cache/screenshots/{os}/{project}/current/`
2. First run: no baselines exist, so current screenshots become baselines automatically
3. Subsequent runs: pixel-level comparison with configurable tolerance and threshold
4. Failed comparisons generate red-highlight diff PNGs in `diffs/` directory
5. `f76`: accept new baselines (promote current to baselines)

**Result:** Visual regression testing in a 314KB binary. No Node.js, no Playwright, no browser dependency. Catches CSS regressions that unit tests miss.

**Named:** Sim 4 Visual Regression Orchestrator
**Commit:** `8724b66`
**Origin:** Quality assurance processes in defense contracts — visual inspection of hardware builds against engineering drawings. Applied to web UI: the screenshot is the "engineering drawing," the diff is the inspection report.

---

## Entries

### 2026-04-09 — MSRV Bump to 1.85 + Human Revelations Documentation Pass

**What:** Two changes shipped in the current sprint. (1) Bumped `rust-version` from an older floor to `1.85` in `Cargo.toml` — matches the edition 2024 floor. Edition 2024 was already declared but MSRV lagged behind; a build on a 1.84 toolchain would have produced a misleading error. (2) Added the **Human Revelations** section to this document: Triple Sims, Two-Binary Model, and Sim 4 Visual Regression Orchestrator — each with Problem/Insight/Technique/Result/Origin. These are the human-invented ideas that distinguish the exopack from generic AI-assisted glue code.
**Why:** Edition and MSRV must move together — otherwise the toolchain error message is the wrong one. And the Human Revelations section is the provenance record: which techniques came from a human pilot vs. which were routine code generation. Without it, the timeline looks like any other "AI wrote a library" story.
**Commits:** `33ceb9e` (MSRV bump), `ef0eb32` (Human Revelations section)
**AI Role:** AI made the mechanical edits (Cargo.toml version field, formatting the Human Revelations prose). Human identified which techniques were genuinely novel, provided the origin stories (weapons qualification, embedded firmware self-test, defense-contract visual inspection), and chose the MSRV target.

### 2026-04-02 — Standards Check: Rust Industry Standards Quality Gate

**What:** New `standards_check` module (f100–f116, t70–t72). 14 checks per project: clippy, fmt, audit, deny, MSRV, unsafe, docs, changelog, license, test binary (P16), allow(unused), error handling, secrets, Cargo.toml metadata. Portfolio integration test runs all 14 checks across 10 cochranblock projects (140 total checks). 16 unit tests for individual check logic. First run: 72/140 passed — exposed gaps across the portfolio.
**Why:** Need a single quality gate for the whole portfolio. Every project should meet Rust community standards.
**Commit:** (this commit)
**AI Role:** AI designed and implemented the module, ran against all projects. Human directed scope.

### 2026-03-31 — Truth Audit: Supply Chain Security + File Cleanup

**What:** Federal-grade supply chain audit: `cargo audit` (1 vuln: idna 0.3.0, non-exploitable on localhost), `cargo outdated` (all direct deps current), deep code review (0 unsafe in exopack, tokio 1012 expected). File cleanup: removed unused 277KB Nunito font, empty examples/ dir, untracked release binaries from git. Fixed stale metrics in POA (LOC 1584→1781, binary 314KB→362KB, functions 28→27).
**Why:** EO 14028 supply chain verification. Every number in every doc must match reality.
**Commit:** `521af17`
**AI Role:** AI ran full audit toolchain, diagnosed findings, performed cleanup. Human directed audit scope.

### 2026-03-29 — Multi-Arch: macOS ARM + Linux x86_64

**What:** Built exopack for two architectures. macOS ARM (362 KB) built locally. Linux x86_64 (384 KB) built on st (kova-elite-support) via vendored deps + rsync. Both uploaded to GitHub Release v0.1.0. Build script at `scripts/build-targets.sh`.
**Why:** CI servers run Linux. Dev machines run macOS ARM. Need both.
**Commit:** `b3817f3`
**AI Role:** AI wrote build script, handled workspace conflict on st (/tmp build), uploaded to release.

### 2026-03-28 — Crates.io Metadata + Dogfood Govdocs CLI

**What:** Added crates.io publish metadata (license=Unlicense, repository, keywords, categories). `cargo publish --dry-run` passes. Dogfood: `exopack govdocs [topic]` subcommand prints all 11 compliance docs from binary (baked via `include_str!`). `exopack govdocs deps` parses baked Cargo.toml at runtime. `exopack --sbom` outputs SPDX 2.3 format for federal scanners.
**Why:** The binary IS the compliance artifact. No external docs needed.
**Commits:** `542611b`, `036bd11`
**AI Role:** AI implemented CLI subcommands and SPDX output. Human directed dogfooding approach.

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
