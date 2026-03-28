<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Federal Use Cases — exopack

*Which agencies could use this project and how. 2026-03-27.*

## Overview

exopack is a testing augmentation library for Rust binaries. Its value to federal agencies is in **software quality assurance** — specifically, deterministic testing (TRIPLE SIMS), visual regression (Sim 4), and the two-binary model that keeps test deps out of production.

---

## DoD (Department of Defense)

**Use case:** Software assurance for Rust-based cyber tools and infrastructure.

- **TRIPLE SIMS** for weapon system software testing — 3-pass determinism catches race conditions that single-pass testing misses
- **Visual regression** for C2 (command and control) dashboards — detect unintended UI changes in classified interfaces
- **Two-binary model** aligns with DoD DevSecOps Reference Design: test binary with full instrumentation, production binary stripped clean
- **Zero external deps** in core (triple_sims) — no supply chain risk for air-gapped environments

## DHS (Department of Homeland Security)

**Use case:** Quality gates for border security and fraud detection web applications.

- **Visual regression** catches UI tampering or unauthorized changes in citizen-facing portals
- **Mock server** (WireMock) enables testing against simulated external APIs (CBP, ICE systems) without connecting to production
- **Screenshot capture** provides audit evidence of application state at test time

## VA (Department of Veterans Affairs)

**Use case:** Testing health record and benefits web applications.

- **Visual regression** for VA.gov pages — Section 508 testing: screenshot comparison ensures accessibility features aren't broken by code changes
- **Demo record/replay** captures user workflows (form submissions, navigation) for automated regression testing
- **Baked demo** exercises all endpoints without human input — useful for overnight CI runs

## DOJ (Department of Justice)

**Use case:** Testing case management and legal research tools.

- **TRIPLE SIMS** for data integrity — run tests 3x to verify case data handling is deterministic
- **Mock server** for testing integrations with PACER, FBI databases without real connections
- **Two-binary model** ensures no test code (mock data, test fixtures) ships in production deployments

## NASA / DOE / NSF (Scientific Computing)

**Use case:** Testing scientific computing dashboards and data visualization.

- **Visual regression** for scientific visualization web apps — detect rendering changes in charts, maps, simulations
- **Screenshot comparison** with configurable tolerance — scientific visualizations may have anti-aliasing differences across platforms
- **TRIPLE SIMS** for numerical reproducibility testing — run computation 3x, verify deterministic output

## GSA (General Services Administration)

**Use case:** Shared services and procurement platform testing.

- **Visual regression** for SAM.gov, GSA Advantage, and other procurement portals
- **Mock server** for testing integrations with payment processors and vendor APIs
- **Baked demo** as automated smoke test for all procurement workflows after deployment
- **SBOM and supply chain documentation** already provided — meets GSA's software transparency requirements

---

## Cross-Agency Value

| Capability | Federal Need |
|-----------|-------------|
| TRIPLE SIMS (3-pass determinism) | FISMA requires verified, reproducible test results |
| Two-binary model | Separation of test and production code reduces attack surface |
| Visual regression | Section 508 compliance testing — detect accessibility regressions |
| SBOM + supply chain docs | EO 14028 compliance for all federal software |
| Zero-dep core | Deployable in air-gapped and FedRAMP-authorized environments |
| Rust-native | Memory safety without runtime overhead — aligns with NSA Cybersecurity Information Sheet on memory-safe languages |
