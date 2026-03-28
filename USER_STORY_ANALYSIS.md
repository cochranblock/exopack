<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# User Story Analysis — exopack

*Full end-to-end walkthrough from a new user's perspective. 2026-03-27.*

---

## 1. Discovery

**First impression from README:** Clear. Within 10 seconds I know: "testing augmentation for Rust binaries" — screenshots, mocks, TRIPLE SIMS. The mermaid diagram shows how it fits into a workspace. Feature list is scannable.

**What's unclear:** The zero-cloud banner at the top takes up 9 lines before the product name. A user scanning GitHub has to scroll past marketing to find what this actually does. The compressed function names (f60, t61) in source code will confuse anyone browsing on GitHub who doesn't know P13.

**Grade: 7/10** — Good signal-to-noise ratio once you get past the banner.

---

## 2. Installation

**As a library (primary use case):**
```toml
exopack = { git = "https://github.com/cochranblock/exopack", features = ["triple_sims", "screenshot"] }
```
No install step needed. Feature gates mean you only pull what you need. This is good.

**As a CLI binary:**
```
cargo build --release --features triple_sims
./target/release/exopack --help
```
Works. Binary is 314 KB. Help text is now clear (after fixes).

**Friction:** README doesn't show the git dependency line. A user has to figure out `features = ["triple_sims"]` from reading Cargo.toml. No "Quick Start" section.

**Grade: 6/10** — Works, but no install/quickstart docs.

---

## 3. First Use — Happy Path

**Scenario:** I have a Rust project `myapp` and want to run its test binary 3 times.

Step 1: Add exopack dep with `triple_sims` feature. OK.
Step 2: In my test binary, call `exopack::triple_sims::f60(|| run_tests()).await`. OK.
Step 3: `cargo run --bin myapp-test --features tests`

**Or via CLI:**
```
exopack live-demo ./myapp --features tests
```
This auto-discovers the `-test` binary from Cargo.toml and runs it with live output. It works.

**Friction:** The function is `f60`, not `triple_sims_run`. A new user reads the README, sees "triple_sims — Run test runner 3 times", goes to the code, and finds `pub async fn f60`. They need to look at the doc comment or compression map to understand it. The compression map is a kova-internal convention, not a Rust community convention.

**Grade: 5/10** — Works, but P13 naming is hostile to new users.

---

## 4. Second Use Case — Screenshot Comparison

**Scenario:** I want visual regression testing for my web app.

Step 1: Add `features = ["screenshot"]`.
Step 2: Call `exopack::screenshot::f73(base_url, project, pages, 10, 1.0).await`.
Step 3: First run creates baselines. Second run compares.

**What works:** The auto-baseline-on-first-run is great UX. Diff images highlighting changes in red are useful. `f76` to accept new baselines is a clean workflow.

**Friction:** The function is `f73`, not `visual_regression`. Every screenshot function is `f70`, `f71`, `f72` — zero discoverability. IDE autocomplete shows `f70, f71, f72, f73, f76, f77, f78, f79`. No way to know which is which without reading doc comments.

**Grade: 5/10** — Good feature, bad API ergonomics.

---

## 5. Edge Cases

| Input | Expected | Actual | Verdict |
|-------|----------|--------|---------|
| `exopack --help` | Show usage, exit 0 | Shows usage, exit 0 | PASS (fixed) |
| `exopack --version` | Show version | Shows "exopack 0.1.0" | PASS (fixed) |
| `exopack live-demo /nonexistent` | "dir not found" | "Cargo.toml not found in /nonexistent" | PASS (fixed) |
| `exopack live-demo /tmp` | "no Cargo.toml" | "Cargo.toml not found in /tmp" | PASS |
| `exopack live-demo .` | "no -test binary" | "No *-test binary found in ." | PASS |
| `exopack foobar` | "unknown command" | "Unknown command: foobar" | PASS |
| `exopack ""` | reasonable error | "Unknown command: " (empty) | WEAK — empty string not caught |
| `exopack live-demo . nonexistent-bin` | "binary not found" | Cargo error during build | PASS — cargo gives the error |

**Grade: 7/10** — Error messages are clear after fixes. No crashes.

---

## 6. Feature Gap Analysis

Things a real user would expect:

1. **`exopack test <project_dir>`** — Run TRIPLE SIMS directly from CLI, not just `live-demo`
2. **`exopack screenshot <url> <output>`** — Capture a single screenshot from CLI
3. **`exopack compare <a.png> <b.png>`** — Compare two images from CLI
4. **`exopack init`** — Add exopack to an existing project's Cargo.toml
5. **CI integration examples** — GitHub Actions / GitLab CI YAML showing exopack in a pipeline
6. **Baseline management** — `exopack baselines update <project>` from CLI
7. **JSON test report output** — Machine-readable results for CI systems
8. **Parallel test execution** — Run TRIPLE SIMS passes in parallel (for independent tests)

---

## 7. Documentation Gaps

Questions a user would have that docs don't answer:

1. How do I add exopack as a dependency? (No Cargo.toml snippet)
2. What does a minimal test binary look like? (No code example)
3. How do I use screenshot + devtools together? (Feature interaction unclear)
4. What Chromium version does devtools download? (Not documented)
5. How do I run visual regression in CI without a display? (Headless setup)
6. What's the diff between `f79` (capture_project) and `f75` (capture_screenshots)? (Confusing overlap)
7. What are f60, t61, s70? (P13 not explained in README)

---

## 8. Competitor Check

| Feature | exopack | cargo-nextest | insta | lychee |
|---------|---------|--------------|-------|--------|
| Test runner | TRIPLE SIMS (3x) | Parallel, retry | Snapshot | N/A |
| Visual regression | Pixel diff + baselines | No | Text snapshots | No |
| Mock server | WireMock | No | No | No |
| Screenshot | Headless Chromium + Pure Rust | No | No | No |
| Binary model | Two-binary (prod + test) | Uses cargo test | Uses cargo test | N/A |
| Maturity | New (v0.1.0) | Stable | Stable | Stable |
| Community | Solo project | Widely adopted | Widely adopted | Moderate |

**Honest assessment:** exopack's two-binary model and TRIPLE SIMS are genuinely novel. The visual regression is solid. But it's a private toolkit for the CochranBlock ecosystem, not a general-purpose tool. The P13 compression makes the API impractical for outside users. Competitors have better docs, more users, and standard Rust naming.

**Where exopack wins:** All-in-one testing augmentation. No other single crate gives you screenshots + mocks + visual regression + determinism testing in one feature-gated package.

---

## 9. Verdict

| Category | Score | Notes |
|----------|-------|-------|
| Usability | 5/10 | P13 naming tanks discoverability. Works once you know the map. |
| Completeness | 7/10 | Core features solid. CLI needs more commands. |
| Error Handling | 8/10 | Clean errors, no crashes, good exit codes (after fixes). |
| Documentation | 4/10 | No quickstart, no code examples, no CI guide. Compression map exists but isn't user-friendly. |
| Would Pay For | 3/10 | As-is, no. The two-binary model + visual regression are interesting but not packaged for external consumption. |

**Overall: 5.4/10** — Solid internals, weak surface. A power tool for its author, not a product for the market.

---

## 10. Top 3 Fixes (Implemented)

### Fix 1: `--help` and `--version` flags
**Before:** `exopack --help` → "Unknown subcommand: --help"
**After:** Proper help text with version, commands, examples. Exit 0.

### Fix 2: Validation order — check dir exists before parsing Cargo.toml
**Before:** `exopack live-demo /nonexistent` → "No -test binary found in Cargo.toml"
**After:** "Cargo.toml not found in /nonexistent"

### Fix 3: Better error messages with context
**Before:** "No -test binary found in Cargo.toml. Specify bin_name explicitly."
**After:** "No *-test binary found in ./myproject. Specify bin_name explicitly." + usage hint with the actual path.

---

*Analysis by Claude Opus 4.6 — 2026-03-27. Scores are honest.*
