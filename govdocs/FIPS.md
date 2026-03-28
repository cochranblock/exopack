<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# FIPS 140-2/3 Status — exopack

*2026-03-27.*

## Crypto Primitives Used

exopack does **not implement any cryptographic algorithms**. It is a testing library.

| Primitive | Used? | Where | FIPS Status |
|-----------|-------|-------|-------------|
| AES | No | — | — |
| RSA | No | — | — |
| SHA-256 | No | — | — |
| HMAC | No | — | — |
| HKDF | No | — | — |
| Argon2 | No | — | — |
| TLS | Indirect | reqwest → rustls (screenshot/interface features) | Not FIPS-validated |

## TLS Stack

When the `screenshot` or `interface` features are enabled, `reqwest` uses `rustls` for HTTPS. rustls is **not FIPS 140-2/3 validated**.

However, exopack only makes HTTP requests to `localhost` test servers (127.0.0.1). TLS is used by the reqwest client but in practice all connections are plaintext HTTP to local ports.

## Path to FIPS Compliance

If FIPS-validated TLS were required:
1. Replace `rustls` with `openssl` feature in reqwest (links to system OpenSSL, which can be FIPS-validated)
2. Or: since all connections are localhost, TLS could be disabled entirely

Neither change is currently needed because exopack makes no external network connections in its normal operation.

## Classification

exopack is a **build-time development tool** that does not process, transmit, or store any data requiring cryptographic protection. FIPS 140-2/3 certification is not applicable to this project's use case.
