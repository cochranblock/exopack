<!-- Unlicense — cochranblock.org -->

# Backlog

*Prioritized stack. Most important at top. Max 20 items.*
*Last reorganized: 2026-04-03*

---

1. [fix] Raise exopack standards score from 6/14 — add `rust-version`, run `cargo fmt`, remove unjustified `#[allow(unused)]` in lib.rs, eliminate 1 unwrap in lib code
2. [feature] Checkpoint/undo test harness (f383/f384 coverage) — verify sled snapshot before write, restore on undo. Depends: kova `tools.rs` checkpoint API stable
3. [feature] Context compaction test harness (f380 coverage) — verify summarization preserves key facts, token count drops below budget. Depends: kova `context_mgr.rs`
4. [feature] Dual-mode inference test harness (f382 coverage) — mock local GGUF and remote Anthropic routing, verify env-driven dispatch. Depends: kova `inference/mod.rs`
5. [feature] Exec tool + permission gate test harness — verify open/guarded modes, tool rename backward compat. Depends: kova `tools.rs` exec rename
6. [build] Upgrade reqwest 0.11 → 0.12+ — breaking change (async body types), affects interface + screenshot + baked_demo features
7. [fix] Address idna 0.3.0 advisory — update transitive dep chain or pin to safe version
8. [test] Add async test coverage — f60 (triple_sims runner), f80/f81 (interface), f82-f87 (mock server). Currently 0 async tests
9. [feature] Standards check: add `cargo outdated` as 15th check — flag stale direct deps across portfolio
10. [fix] Standards check: workspace src/ detection for approuter/rogue-repo `unsafe`/`allow_unused`/`error_handling` returns "no src/" instead of checking inner crate
11. [test] Integration test: wire exopack standards_check into a consuming project's test binary (cochranblock-test or kova-test)
12. [build] `cargo publish` to crates.io — dry-run passes, metadata complete, just needs the push
13. [docs] Update govdocs/SBOM.md with standards_check module, new dep count, new function count
14. [feature] Add `exopack standards [project_dir]` CLI subcommand — run standards check from the binary, not just tests
15. [research] Nanobyte priority queue design — informed by sled DB audit (5 trees in kova, prefix-based in cochranblock). Needs: priority-ordered keys, peek/pop, TTL
16. [fix] Portfolio-wide: add `rust-version = "1.75"` to all 10 Cargo.toml files (currently 0/10 pass MSRV check)
17. [fix] Portfolio-wide: add `#![forbid(unsafe_code)]` to projects with 0 unsafe usage (pixel-forge, wowasticker, approuter, ronin-sites)
18. [build] Multi-arch release v0.2.0 — rebuild macOS ARM + Linux x86_64 after reqwest upgrade + standards_check feature
19. [research] IRONHIVE integration — can exopack's standards gate run distributed across the cluster? One node per project, parallel check
20. [docs] Update USER_STORY_ANALYSIS.md — re-score with standards_check, re-run the 10-point walkthrough post-improvements
