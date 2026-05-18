# Assumed Breach Threat Model

> **Operating assumption: every component below is already compromised. Design for damage containment and loud detection, not for prevention.**

This document is the canonical threat model for every project in the `cochranblock/*` portfolio. Each project adapts the Threat Surface section for its own context but shares the same first principles, mitigations, and verification protocol.

---

## First Principles

1. **Every record that matters has an external witness.** Hashes published to public git (or equivalent neutral timestamp authority) so tampering requires simultaneously corrupting your system AND the public chain.
2. **No single point of compromise.** Signing keys in hardware (YubiKey / TPM / Secure Enclave). Never in software. Never in env vars. Never in config files.
3. **Default air-gap.** No network dependency for correctness. Network is for backup + publishing hashes, both signed, both verifiable post-hoc.
4. **Append-only everything.** No delete path in any storage layer. Corrections are reversing entries referencing the original. Standard accounting discipline, enforced in code.
5. **Cryptographic audit chain.** Every day's state derives from the previous day's hash. Tampering with any day invalidates every subsequent day.
6. **Disclosure of methodology is a security feature.** If an auditor can independently verify the algorithm, they can independently verify the outputs. No "trust us" layers.
7. **Separation of duties enforced in software.** Entry, approval, and audit live in different trust zones. Compromise of one does not compromise the others.
8. **Redundancy across trust zones.** Local + different-cloud + different-format + offline. Attacker must compromise all to hide damage.
9. **Test breach scenarios regularly.** Triple Sims applied to tamper detection. If the chain does not detect a simulated tamper, the chain is broken.

---

## Threat Surface (exopack-specific)

exopack is a **testing augmentation library** consumed by every project in the portfolio. It spawns processes, binds network ports, drives headless browsers, and produces test evidence (screenshots, diff images, pass/fail results, standards reports). It emits no financial, legal, or user-data records. Its threat surface is the supply-chain trust surface of the CI pipeline itself.

### Applicable threats

| Threat | exopack-specific risk |
|--------|-----------------------|
| **Supply chain (deps)** | exopack runs in every project's `*-test` binary. A backdoored exopack dependency poisons all 10 portfolio projects' CI at once. Highest-leverage target in the portfolio. |
| **Binary compromise** | If the exopack library or `exopack` CLI is tampered, every test result is a lie. Visual regressions pass when UI is broken, standards checks pass when code is non-compliant, triple sims pass when tests are flaky. |
| **Screenshot baseline poisoning** | Adversarial commit replaces baseline PNGs with screenshots of a broken UI. All subsequent visual regressions silently accept the broken state as "correct." |
| **Standards check output tampering** | If the 14-point standards gate (clippy, fmt, audit, deny, MSRV, unsafe, etc.) is tampered to always-pass, the portfolio's quality floor collapses without anyone noticing. |
| **Test harness / network exposure** | `interface` spawns HTTP servers on random ports. `devtools` launches headless Chromium via CDP. `mock` runs WireMock stubs. During CI, these bind real network interfaces. An attacker on the CI machine's network can inject responses or exfiltrate test payloads. |
| **Headless Chromium (CDP)** | CDP trusts localhost. If an attacker is on the same host during a devtools-driven screenshot capture, they can inject JS, capture DOM state, or pivot via the debug protocol. |
| **Demo recording replay** | `demo` records action scripts as JSON. If a recorded demo is tampered, `baked_demo` replays wrong actions — could mask a broken workflow by replaying a passing one from a different commit. |
| **Clock manipulation** | Triple sims compare timing across 3 runs. Clock drift or manipulation could cause false flakiness reports or hide real timing bugs. |

### N/A for exopack

| Threat | Why N/A |
|--------|---------|
| **Storage compromise (append-only records)** | exopack stores no records of legal or financial consequence. Screenshots and test results are ephemeral CI artifacts, not audit records. |
| **Signing key theft** | exopack produces no signed release artifacts. Its outputs are test evidence consumed by other projects' CI, not distributed binaries. |
| **Audit log tampering** | No audit log. Test output is stdout/stderr + PNG files, not a structured tamper-evident chain. |
| **Backup tampering** | No backup regime. Test artifacts are regenerated from source on every run. |
| **Insider / self-tampering of legal records** | No legal records. The library is Unlicense / public domain. |
| **Physical device seizure** | No secrets stored. No encryption keys. The repo is fully public. Device seizure yields nothing beyond the public git history. |
| **User account compromise** | No user accounts. No auth. No multi-tenant access. |
| **Hardware-key signing** | N/A. exopack emits no artifacts that require hardware-key provenance. |
| **Public-chain deployment** | N/A. No daily records of consequence to publish. The test artifacts are derived entirely from source and regenerated on every CI run. |

### Key mitigation specific to exopack

**The test tool must not be tested by itself.** exopack's standards_check runs against other projects, not against exopack. If exopack validates exopack, that is a self-licking ice cream cone (anti-pattern). Cross-project validation is the only credible path: if cochranblock's tests pass but kova's fail on the same exopack version, something is wrong with exopack, not with kova.

---

## Mitigations

| Assume | Mitigation | Verification |
|--------|-----------|--------------|
| Binary compromised | Hardware-key signatures for every output of consequence | Anyone can verify the public key matches expected fingerprint |
| Storage compromised | Append-only sled trees. Delete is not a function, not a policy. | Hash chain breaks on any rewrite. External witness detects. |
| Network MITM | Air-gap capable. Network used only for signed backups + hash publishing. | NTP + GitHub timestamp + hardware counter cross-checked. |
| Signing key stolen | Daily hash committed to public git. Stolen key cannot retroactively change committed days. | Any day older than the public commit is immutable in evidence. |
| Audit log tampered | Separate sled tree, write-only from main app. Auditor tool reads both + cross-checks. | Compromise of main app leaves audit log intact. |
| Backup tampered | 3 different targets with 3 different credentials (local USB + off-site cloud + paper). | Attacker needs all three to hide damage. |
| Insider / self-tampering | No admin role. No delete. Reversing entries only. | Legal record immune to author second-thoughts. |
| Clock manipulation | Multiple time sources: local clock, NTP, git commit timestamp, hardware-key counter. | Divergence flags exception requiring supervisor approval. |
| Supply chain (deps) | `cargo audit` in CI. Pinned SBOM. Reproducible builds where possible. | Anyone can reproduce the binary from source + lockfile. |
| Physical device seizure | Full-disk encryption. Hardware key physically separate from device. | Stolen laptop without key is useless for forgery. |

---

## Public-Chain Deployment

This project publishes tamper-evident hashes to a public companion repo: `cochranblock/<project>-chain` (where `<project>` is the project name).

- **Daily cycle:** at 23:59 local, compute BLAKE3 of all records-of-consequence from the day. Sign with hardware key. Commit to chain repo. Push.
- **GitHub timestamp** on the commit = neutral third-party witness. Anyone can cold-verify records were not rewritten after commit time.
- **Verification:** `<project> verify` reads the chain and re-derives hashes. Any divergence = tampering detected.

This pattern is a private Certificate Transparency log for project state. Same primitive Google uses for TLS certs, applied to whatever the project tracks.

---

## Triple Sims for Tamper Detection

Standard Triple Sims gate (run 3x identically) extended with a tamper-scenario sim:

1. Normal run → produce canonical output
2. Simulated tampering (flip one bit in storage) → `verify` must flag it
3. Simulated clock rewind → `verify` must flag it

If any sim fails to detect, the chain is broken. Fix before merge.

---

## Scope of this Document

- Covers: any artifact this project emits that has legal, financial, or audit consequence.
- Does NOT cover: source code itself (public under Unlicense, not sensitive), build outputs (reproducible), marketing content (public by design).
- If your project emits no records of consequence, the relevant sections are zero-length and the public-chain deployment is skipped. Document that explicitly.

---

## Relation to Other Docs

- **TIMELINE_OF_INVENTION.md** — establishes priority dates for contributions. Feeds into the chain's initial state.
- **PROOF_OF_ARTIFACTS.md** — cryptographic signatures on release artifacts. Adjacent pattern, same first principles.
- **DCAA_COMPLIANCE.md** (where applicable) — how this threat model satisfies FAR/DFARS audit requirements.

---

## Status

- [x] Threat Surface section adapted for this project
- [ ] Hardware-key signing integrated or N/A documented
- [ ] Public-chain repo created and connected or N/A documented
- [ ] Triple Sims tamper-detection test present or N/A documented
- [ ] External verification procedure documented

---

*Unlicensed. Public domain. Fork, strip attribution, adapt, ship.*

*Canonical source: cochranblock.org/threat-model — last revision 2026-04-14*
<!-- COCHRANBLOCK-BRAND-FOOTER:START - generated by cochranblock/scripts/brand-stamp.sh -->

---

<sub>&#9656; **THE COCHRAN BLOCK, LLC** &#183; CAGE `1CQ66` &#183; UEI `W7X3HAQL9CF9` &#183; UNLICENSE &#183; [cochranblock.org](https://cochranblock.org)</sub>
<!-- COCHRANBLOCK-BRAND-FOOTER:END -->
