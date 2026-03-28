<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Export Control Classification — exopack

*ITAR/EAR analysis. 2026-03-27.*

## ITAR (International Traffic in Arms Regulations)

exopack is **not ITAR-controlled**. It is not designed, developed, or modified for military applications. It is a general-purpose software testing library.

- Not on the United States Munitions List (USML)
- No military-specific functionality
- No classified information
- No defense articles or services

## EAR (Export Administration Regulations)

### EAR Category 5 Part 2 — Encryption

exopack does **not implement encryption**. It does not contain:
- Symmetric encryption (AES, ChaCha20, etc.)
- Asymmetric encryption (RSA, ECC, etc.)
- Key exchange protocols
- Custom cryptographic algorithms

The only crypto-adjacent functionality is the TLS client in reqwest (via rustls), which is:
- An existing published crate available worldwide on crates.io
- Used only for localhost HTTP connections in practice
- Already publicly available and not subject to EAR encryption controls per the "publicly available" exception (EAR §742.15(b))

### Classification

exopack is classified as **EAR99** — a catch-all for items not specifically controlled by any other category. EAR99 items:
- Do not require an export license for most destinations
- Are subject to general prohibitions (sanctioned countries, denied persons)
- Can be freely shared as open-source software

### Open Source Exception

Under EAR §734.7, publicly available software (open source) is not subject to EAR when:
1. It is published and available to the public without restriction ✓ (GitHub, Unlicense)
2. It does not contain controlled encryption source code ✓ (no custom crypto)

exopack qualifies for this exception.

## Summary

| Regulation | Status | Classification |
|------------|--------|---------------|
| ITAR | Not controlled | Not a defense article |
| EAR | EAR99 | General-purpose software, publicly available |
| EAR Cat 5 Part 2 | Not applicable | No encryption implemented |
