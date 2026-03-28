<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# NIST SP 800-218 SSDF Compliance — exopack

*Secure Software Development Framework mapping. 2026-03-27.*

## PS: Prepare the Organization

| Task | Status | Evidence |
|------|--------|----------|
| PS.1: Define security requirements | Done | Feature-gated architecture — each module isolated. No default deps. |
| PS.2: Implement roles and responsibilities | Done | Single-maintainer model. All commits signed. AI-assisted with human review. |
| PS.3: Implement supporting toolchains | Done | Rust compiler (memory-safe), cargo clippy (lint), cargo audit (vuln scan). |

## PW: Protect the Software

| Task | Status | Evidence |
|------|--------|----------|
| PW.1: Design software to meet security requirements | Done | Zero-dep binary for core (triple_sims). Feature gates prevent unnecessary code inclusion. `panic = 'abort'` in release — no unwinding attack surface. |
| PW.2: Review the software design | Done | Architecture documented in `docs/testing_architecture.md` (2,286 lines). Two-binary model separates test deps from production. |
| PW.4: Reuse existing well-secured software | Done | Uses wiremock (established mock library), tokio (industry-standard async), image crate (widely audited). No custom crypto. |
| PW.5: Create source code by adhering to secure coding practices | Done | `cargo clippy -- -D warnings` enforced. No `unsafe` blocks in exopack source. No raw pointer manipulation. |
| PW.6: Configure the compilation to diminish vulnerabilities | Done | `opt-level = 'z'`, `lto = true`, `strip = true`, `panic = 'abort'`. Release binary stripped of debug symbols. |
| PW.7: Review and/or analyze human-readable code | Done | All code in public GitHub repo. P13 compression map documents every public symbol. |
| PW.8: Test executable code | Done | 17 unit tests across 4 modules. TRIPLE SIMS (3-pass determinism) is the core testing philosophy. |
| PW.9: Configure software to have secure settings by default | Done | Default features are empty — zero deps pulled unless explicitly opted in. No network access in default configuration. |

## RV: Respond to Vulnerabilities

| Task | Status | Evidence |
|------|--------|----------|
| RV.1: Identify and confirm vulnerabilities | Partial | `cargo audit` can scan. No automated CI pipeline yet (project uses embedded test binary model, not GitHub Actions). |
| RV.2: Assess, prioritize, and remediate vulnerabilities | Ready | Cargo.lock pins all versions. `cargo update` + audit cycle for remediation. |
| RV.3: Analyze vulnerabilities to identify root causes | Ready | Git blame + commit history available. Timeline of Invention documents all changes. |

## PO: Protect Operations

| Task | Status | Evidence |
|------|--------|----------|
| PO.1: Verify third-party components | Done | SBOM in `govdocs/SBOM.md`. All deps from crates.io with version pinning. |
| PO.2: Provide a mechanism for verifying software integrity | Done | Cargo.lock committed. Deterministic builds via `codegen-units = 1` + `lto = true`. |
| PO.3: Archive and protect software releases | Done | All source in git. GitHub provides release archival. |
