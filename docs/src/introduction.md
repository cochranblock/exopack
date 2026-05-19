# exopack

Test augmentation library for The Cochran Block stack. Deterministic fixtures, two-binary tripwire, harvest pipeline.

## What It Is

exopack is the shared test infrastructure used across cochranblock projects. It provides:

- **ATS fixtures** — deterministic fake job postings and application data for testing atsisbroken
- **Harvest pipeline** — scrape → normalize → store pipeline for building test datasets
- **Two-binary tripwire** — integration test pattern that runs two independent binaries and compares output
- **Triple sims gate** — simulation-based quality gate used before marking sprints done

## Workspace

| Crate | Purpose |
|-------|---------|
| `exopack` | Core fixture types and generation |
| `exopack-harvest` | Data harvest and normalization pipeline |
| `wasm-smoke` | WASM smoke test runner |
<!-- COCHRANBLOCK-BRAND-FOOTER:START -->

---

<sub>&#9656; **THE COCHRAN BLOCK, LLC** &#183; CAGE `1CQ66` &#183; UEI `W7X3HAQL9CF9` &#183; UNLICENSE &#183; [cochranblock.org](https://cochranblock.org)</sub>
<!-- COCHRANBLOCK-BRAND-FOOTER:END -->
