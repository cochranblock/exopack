<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Supply Chain Integrity — exopack

*2026-03-27.*

## Dependency Sources

All dependencies come from **crates.io**, the official Rust package registry. No vendored binaries. No private registries. No git submodules with binary blobs.

## Version Pinning

`Cargo.lock` is committed to the repository. Every transitive dependency is pinned to an exact version. Builds are reproducible: same `Cargo.lock` + same Rust toolchain = same binary.

## Build Reproducibility

Release profile settings that aid reproducibility:

```toml
[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1    # Single codegen unit = deterministic output
panic = 'abort'
strip = true
```

`codegen-units = 1` forces the compiler to produce consistent output across builds. LTO performs whole-program linking. These together reduce non-determinism.

## No Vendored Binaries

- No pre-compiled `.so`, `.dylib`, `.dll`, or `.wasm` files in the repository
- The `fonts/` directory contains only font files (Nunito TTF) — these are data, not executable code
- chromiumoxide may download a Chromium binary at runtime (devtools feature only) — this is fetched from the official Chromium distribution, not vendored

## Source Availability

All source code is public at `https://github.com/cochranblock/exopack` under the Unlicense (public domain). Every line is auditable.

## Verification Commands

```bash
# Verify all deps resolve from crates.io:
cargo verify-project

# Audit for known vulnerabilities:
cargo audit

# Check for yanked crates:
cargo audit --deny yanked

# Verify no binary files in source:
find . -name "*.so" -o -name "*.dll" -o -name "*.dylib" -o -name "*.wasm" | head

# Verify Cargo.lock integrity:
cargo generate-lockfile --check
```

## Transitive Dependency Count

- Default features (triple_sims only): **0 external deps**
- All features enabled: **17 direct deps**, ~200 transitive (Rust ecosystem standard for async HTTP + image processing)
