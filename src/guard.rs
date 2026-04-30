// Unlicense — public domain — cochranblock.org
//! Two-binary build guards.
//!
//! The two-binary model says: production binary = no test code, test binary
//! = same crate with a `tests` feature gate. The catastrophic mistake is
//! `cargo build --release --features tests` — that ships test internals
//! (mock endpoints, demo data, debug routes) at production speed.
//!
//! [`deny_release_with_tests!`] is a compile-time tripwire a consumer drops
//! into their `*-test/main.rs`. It fails the build when the consumer's
//! `tests` feature is enabled in a release profile, before any test code
//! can land in a release artifact.

/// Compile-time tripwire — invoke from `*-test/main.rs`. Fails the build if
/// the consumer's `tests` feature is enabled in release profile.
///
/// ```ignore
/// // in src/bin/myapp-test.rs
/// exopack::deny_release_with_tests!();
///
/// fn main() {
///     // … test runner …
/// }
/// ```
///
/// Mechanism: `debug_assertions` is on in dev/test profiles and off in
/// release. The intersection of `feature = "tests"` and a release profile
/// is the dangerous combo.
#[macro_export]
macro_rules! deny_release_with_tests {
    () => {
        #[cfg(all(feature = "tests", not(debug_assertions)))]
        const _: () = {
            ::core::compile_error!(
                "exopack tripwire: cannot build the *-test binary in release profile \
                 with the `tests` feature enabled. A release+tests build would ship \
                 test internals (mock endpoints, debug routes) as a production binary. \
                 Build the *-test binary in dev profile, or build the production binary \
                 without --features tests."
            );
        };
    };
}

#[cfg(test)]
mod tests {
    // Invoke at module scope. `cargo test` runs in dev profile, so
    // `debug_assertions` is on and the cfg-gated compile_error is skipped —
    // this compiling at all proves the macro is well-formed.
    crate::deny_release_with_tests!();

    #[test]
    fn macro_compiles_in_dev_profile() {
        assert!(cfg!(debug_assertions), "tests run in dev profile by default");
    }
}
