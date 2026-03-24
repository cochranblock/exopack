<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

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
