<!-- Unlicense — cochranblock.org -->

# Backlog

*Prioritized stack. Most important at top. Max 20 items.*
*Last reorganized: 2026-04-03*

---

1. [build] Upgrade reqwest 0.11 → 0.12+ — breaking change (async body types), affects interface + screenshot + baked_demo features
6. [fix] Address idna 0.3.0 advisory — update transitive dep chain or pin to safe version
7. [test] Add async test coverage — f60 (triple_sims runner), f80/f81 (interface), f82-f87 (mock server). Currently 0 async tests
8. [feature] Standards check: add `cargo outdated` as 15th check — flag stale direct deps across portfolio
9. [fix] Standards check: workspace src/ detection for approuter/rogue-repo `unsafe`/`allow_unused`/`error_handling` returns "no src/" instead of checking inner crate
10. [test] Integration test: wire exopack standards_check into a consuming project's test binary (cochranblock-test or kova-test)
11. [build] `cargo publish` to crates.io — dry-run passes, metadata complete, just needs the push
12. [docs] Update govdocs/SBOM.md with standards_check module, new dep count, new function count
13. [feature] Add `exopack standards [project_dir]` CLI subcommand — run standards check from the binary, not just tests
14. [research] Nanobyte priority queue design — informed by sled DB audit (5 trees in kova, prefix-based in cochranblock). Needs: priority-ordered keys, peek/pop, TTL
15. [fix] Portfolio-wide: add `rust-version = "1.75"` to all 10 Cargo.toml files (currently 1/10 pass MSRV check)
16. [fix] Portfolio-wide: add `#![forbid(unsafe_code)]` to projects with 0 unsafe usage (pixel-forge, wowasticker, approuter, ronin-sites)
17. [build] Multi-arch release v0.2.0 — rebuild macOS ARM + Linux x86_64 after reqwest upgrade + standards_check feature
18. [research] IRONHIVE integration — can exopack's standards gate run distributed across the cluster? One node per project, parallel check
19. [docs] Update USER_STORY_ANALYSIS.md — re-score with standards_check, re-run the 10-point walkthrough post-improvements
