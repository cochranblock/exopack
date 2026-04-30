<!-- Unlicense — cochranblock.org -->

# Backlog

*Prioritized stack. Most important at top. Max 20 items.*
*Last reorganized: 2026-04-29 — v0.3 focusing release*

---

## Just shipped (v0.3 — focusing release)

- [x] Split kova-internal harness modules (checkpoint, compaction, dual_mode, perm_gate) out of exopack core
- [x] Cross-platform harvest: dropped Mac-only paths and the cookie/profile copy; honors `EXOPACK_CHROME_BIN` env var
- [x] Canonical public API names alongside P13 aliases (`triple_sims::run`, `screenshot::visual_regression`, `mock::start_server`, etc.)
- [x] Killed the `chrono_now` lie in `demo.rs` (it was always a UNIX timestamp)
- [x] Sim 4 baseline poisoning fix: first-run captures stage in `baselines_pending/`, require explicit `accept_pending_baselines` to trust
- [x] Sim 4 devtools default + hard-fail when devtools is on (no more silent gray-square placeholder)
- [x] chromiumoxide singleton-lock fix: per-invocation unique `--user-data-dir`
- [x] Two-binary release tripwire — `exopack::deny_release_with_tests!()` macro, compile-errors on release+tests
- [x] Workspace-aware standards_check (unsafe / allow_unused / error_handling now descend into member crates)
- [x] CLI surface: `exopack standards`, `baselines accept`, `screenshot`, `compare` + `--json` for standards
- [x] Dropped `required-features` lock on the `exopack` bin (so `--help`/`govdocs`/`--sbom` work on a bare build); added `cli` umbrella feature
- [x] README quickstart: Cargo.toml snippet, minimal `*-test` binary template, GitHub Actions YAML

## v0.3 ship checklist

1. [build] Bump `version = "0.3.0"` in Cargo.toml; regenerate Cargo.lock
2. [test] Run full lib test suite once on the dev box: `cargo test --features "screenshot,mock,interface,triple_sims,devtools,baked_demo,harvest,standards_check"`
3. [test] Run portfolio standards gate against the 10-project portfolio with the new workspace-aware checks; capture pass-count delta
4. [docs] Update `PROOF_OF_ARTIFACTS.md` v0.3 metrics: module count (9 core + harvest), feature count (10), test count, binary size after `--features cli`
5. [docs] Add a v0.3 entry to `TIMELINE_OF_INVENTION.md` covering: drift split, baseline staging, two-binary tripwire, workspace-aware standards, CLI surface
6. [build] `cargo publish --dry-run`, then `cargo publish` to crates.io (was backlog #7)
7. [release] Cut v0.3.0 multi-arch release via existing CI workflow

## Next (v0.4 — polish)

8. [feature] `EXOPACK_STRICT_BASELINES=1` (or a `VrOptions` struct) — first-run hard-fails instead of staging. Designed cleanly with no env-mutation in tests
9. [feature] `exopack_test_binary!` proc macro — scaffold the *-test bin in 3 lines (the two-binary model is the strongest invention; reduce its boilerplate cost)
10. [feature] 15th standards check: `cargo outdated` (was backlog #4)
11. [feature] Parallel TRIPLE SIMS for explicitly-marked-independent test fns (preserve sequential default — that's the determinism gate)
12. [refactor] Migrate `Result<_, String>` to a typed `ExopackError` (thiserror) — public API hygiene
13. [test] Integration test: wire exopack standards_check into a consuming project's *-test binary (was backlog #6)
14. [docs] Update `govdocs/SBOM.md` for v0.3 dep set (post-harvest split, post-cli umbrella)
15. [docs] Re-run USER_STORY_ANALYSIS — score the v0.3 release; expect Usability up from 5 → 7+, Documentation up from 4 → 6+

## Watch list (no action yet)

- [research] IRONHIVE distribution — can the standards gate run distributed across nodes? Defer until v0.5
- [research] Move `harvest` to its own crate (`exopack-harvest`) once a second consumer appears
- [research] Stable Sim 4 baseline format with semantic versioning so baselines survive exopack upgrades
