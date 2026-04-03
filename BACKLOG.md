<!-- Unlicense — cochranblock.org -->

# Backlog

*Prioritized stack. Most important at top. Max 20 items.*
*Last reorganized: 2026-04-03 — P23 Triple Lens readjust*

---

1. [fix] **`video` always-compiles bug** — `lib.rs:14` has `pub mod video;` with no `#[cfg(feature = "video")]`. Crate fails to compile on platforms without xcap even when video feature is unused. Add the cfg gate; confirm `xcap` dep is already optional in Cargo.toml.

2. [test] **Screenshot core coverage — f71/f72/f73 have zero tests** — Pixel-diff compare (f71), diff-image generation (f72), and full visual_regression orchestrator (f73) are the primary advertised feature with no test coverage. Add: identical PNG → diff=0, different PNG → diff>0, f73 baseline creation on first run, f73 comparison on second run, diff image written for failures. Also add explicit comment in `capture_placeholder` marking it as a stub, not a real capture.

3. [fix] **Upgrade reqwest + patch idna 0.3.0 advisory** — RUSTSEC advisory causes exopack to fail its own standards_check `audit` gate. The quality gate defeats itself. Upgrade reqwest 0.11 → 0.12 (async body types breaking change; update interface + screenshot + baked_demo callers). Verify idna advisory clears after upgrade.

4. [feature] Standards check: add `cargo outdated` as 15th check — flag stale direct deps across portfolio
5. [fix] Standards check: workspace src/ detection for approuter/rogue-repo — `unsafe`/`allow_unused`/`error_handling` returns "no src/" instead of checking inner crate. Real issues in workspace crates go undetected.
6. [test] Integration test: wire exopack standards_check into a consuming project's test binary (cochranblock-test or kova-test)
7. [build] `cargo publish` to crates.io — dry-run passes, metadata complete, just needs the push
8. [docs] Update govdocs/SBOM.md with standards_check module, new dep count, new function count
9. [feature] Add `exopack standards [project_dir]` CLI subcommand — run standards check from the binary, not just tests
10. [research] Nanobyte priority queue design — informed by sled DB audit (5 trees in kova, prefix-based in cochranblock). Needs: priority-ordered keys, peek/pop, TTL
11. [fix] Portfolio-wide: add `rust-version = "1.75"` to all 10 Cargo.toml files (currently 1/10 pass MSRV check)
12. [fix] Portfolio-wide: add `#![forbid(unsafe_code)]` to projects with 0 unsafe usage (pixel-forge, wowasticker, approuter, ronin-sites)
13. [build] Multi-arch release v0.2.0 — rebuild macOS ARM + Linux x86_64 after reqwest upgrade + standards_check feature
14. [research] IRONHIVE integration — can exopack's standards gate run distributed across the cluster? One node per project, parallel check
15. [docs] Update USER_STORY_ANALYSIS.md — re-score with standards_check, re-run the 10-point walkthrough post-improvements
