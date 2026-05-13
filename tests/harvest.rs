// SPDX-License-Identifier: Unlicense
// Unlicense — public domain — cochranblock.org
//! Integration tests for `exopack::harvest` (Gemini sprite CDP automation).
//!
//! The harvest module's happy-path *requires* a logged-in Chrome session
//! on `--remote-debugging-port=9222`. That can't run in CI. These tests
//! cover the unhappy paths and pure-function surface — the parts a
//! consumer can rely on without a browser:
//!
//! - `HarvestConfig::default()` shape
//! - `build_prompt` formatting contract
//! - `default_chrome_bin()` behavior (incl. EXOPACK_CHROME_BIN override)
//! - `find_gemini_ws` returns Err (not panic) when no Chrome is listening
//! - `extract_image` / `harvest_one` return Err on unreachable ws_url

#![cfg(feature = "harvest")]

use exopack::harvest::{
    build_prompt, default_chrome_bin, find_gemini_ws, harvest_one, HarvestConfig,
};
use std::path::PathBuf;

#[test]
fn default_config_has_sane_shape() {
    let c = HarvestConfig::default();
    assert!(c.ws_url.is_empty(), "ws_url defaults to empty until discovered");
    assert_eq!(c.output_dir, PathBuf::from("for_human_review"));
    assert!(!c.class_name.is_empty(), "class_name must default to a real value");
    assert!(c.sheet_count >= 1, "sheet_count must be >= 1");
    assert!(!c.style.is_empty(), "style must default to a real value");
}

#[test]
fn build_prompt_interpolates_both_args_and_is_pure() {
    let a = build_prompt("knight", "Dungeon Crawl Stone Soup");
    let b = build_prompt("knight", "Dungeon Crawl Stone Soup");
    assert_eq!(a, b, "build_prompt must be deterministic");

    assert!(a.contains("knight"), "class must appear in prompt");
    assert!(a.contains("Dungeon Crawl Stone Soup"), "style must appear in prompt");
    assert!(a.contains("5 columns by 6 rows"), "sprite-sheet shape must be in prompt");
    assert!(a.contains("32x32"), "pixel dimension must be in prompt");

    let c = build_prompt("zombie", "Ultima IV");
    assert_ne!(a, c, "different inputs must produce different prompts");
    assert!(c.contains("zombie") && c.contains("Ultima IV"));
}

#[test]
fn build_prompt_passes_through_unusual_input_without_panic() {
    // Don't sanitize — verify we don't accidentally start to.
    let p = build_prompt("", "");
    assert!(p.contains("5 columns by 6 rows"));

    let exotic = build_prompt("ninja\nwith newline", r#"Style with "quotes""#);
    assert!(exotic.contains("ninja\nwith newline"));
    assert!(exotic.contains(r#"Style with "quotes""#));
}

#[test]
fn default_chrome_bin_returns_nonempty_path() {
    // Whatever the platform, default_chrome_bin must return a non-empty
    // path. It's allowed to point at a binary that doesn't exist (Chrome
    // not installed) — that's a runtime concern. The contract here is
    // shape, not existence.
    let p = default_chrome_bin();
    assert!(!p.as_os_str().is_empty(), "must return a non-empty path");
}

#[test]
fn default_chrome_bin_honors_env_override() {
    // SAFETY: only the current process; this test reads/writes a process-
    // local env var. set_var is unsafe in 2024 edition.
    let sentinel = "/tmp/exopack-harvest-test-fake-chrome";
    let prev = std::env::var_os("EXOPACK_CHROME_BIN");
    unsafe {
        std::env::set_var("EXOPACK_CHROME_BIN", sentinel);
    }

    let p = default_chrome_bin();
    assert_eq!(
        p,
        PathBuf::from(sentinel),
        "EXOPACK_CHROME_BIN must override the default"
    );

    // Restore previous env state for clean isolation.
    unsafe {
        match prev {
            Some(v) => std::env::set_var("EXOPACK_CHROME_BIN", v),
            None => std::env::remove_var("EXOPACK_CHROME_BIN"),
        }
    }
}

#[tokio::test]
async fn find_gemini_ws_errors_when_no_cdp_listener() {
    // Pick a port unlikely to be in use. Any failure mode is acceptable
    // as long as it's an Err, not a panic.
    let port: u16 = 59_321;
    let result = find_gemini_ws(port).await;
    assert!(
        result.is_err(),
        "find_gemini_ws must error when nothing listens on port {port}; got: {result:?}"
    );
}

#[tokio::test]
async fn harvest_one_errors_when_ws_url_unreachable() {
    // ws_url that resolves nowhere — harvest_one drives navigate /
    // type / wait / extract through this URL; without a server it
    // must surface an Err, not panic.
    let cfg = HarvestConfig {
        ws_url: "ws://127.0.0.1:1/devtools/page/none".into(),
        output_dir: std::env::temp_dir().join(format!(
            "exopack-harvest-test-{}",
            std::process::id()
        )),
        class_name: "skeleton".into(),
        sheet_count: 1,
        style: "Dungeon Crawl Stone Soup".into(),
    };
    let _ = std::fs::create_dir_all(&cfg.output_dir);

    let result = harvest_one(&cfg, 0).await;
    assert!(
        result.is_err(),
        "harvest_one must error on unreachable ws_url; got: {result:?}"
    );

    let _ = std::fs::remove_dir_all(&cfg.output_dir);
}
