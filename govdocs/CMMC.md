<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# CMMC Compliance Mapping — exopack

*Cybersecurity Maturity Model Certification. 2026-03-27.*

## Applicable Level

exopack is a development tool, not an information system. CMMC practices below describe how exopack **supports** organizations seeking CMMC Level 1-2 compliance in their software supply chain.

## Level 1 Practices (Basic Cyber Hygiene)

| Domain | Practice | exopack Support |
|--------|----------|----------------|
| AC (Access Control) | AC.1.001: Limit system access | Source code is public (Unlicense). No access control needed — no secrets in repo. |
| AC (Access Control) | AC.1.002: Limit to authorized users | GitHub repository access controls. Commits are attributed. |
| IA (Identification & Auth) | IA.1.076: Identify users | Git commit history identifies all contributors. |
| MP (Media Protection) | MP.1.118: Sanitize media | No sensitive data stored. Screenshot cache is test artifacts only. |
| SC (System & Comms) | SC.1.175: Monitor communications | exopack makes no external network connections. localhost only. |
| SI (System & Info Integrity) | SI.1.210: Identify and report flaws | `cargo audit` for vulnerability scanning. SBOM documented. |
| SI (System & Info Integrity) | SI.1.211: Update and patch | Cargo.lock pins versions. `cargo update` + audit for patches. |

## Level 2 Practices (Advanced)

| Domain | Practice | exopack Support |
|--------|----------|----------------|
| AU (Audit) | AU.2.041: Audit events | Git log provides complete audit trail of all code changes. Timeline of Invention documents every feature. |
| AU (Audit) | AU.2.042: Unique audit trail | Git commit hashes are cryptographic (SHA-1). Each commit is uniquely identifiable. |
| CM (Config Management) | CM.2.064: Establish baselines | Cargo.lock pins all dependency versions. Release profile is deterministic. |
| CM (Config Management) | CM.2.065: Track changes | Git history + Timeline of Invention. Every commit dated and described. |
| RM (Risk Management) | RM.2.141: Assess risk | User Story Analysis (`USER_STORY_ANALYSIS.md`) includes security assessment. |
| SA (Security Assessment) | SA.2.171: Evaluate effectiveness | 17 unit tests + TRIPLE SIMS (3-pass determinism) verify correctness. QA rounds documented. |
| SC (System & Comms) | SC.2.179: Use encrypted sessions | rustls for any TLS connections (reqwest). No plaintext credential transmission. |
