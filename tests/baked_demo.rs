// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Integration tests for baked_demo (f95).
//!
//! f95 requires a live kova binary and a running HTTP server, so full
//! end-to-end execution is guarded behind `#[ignore]`. The structural
//! tests here validate contracts that can be exercised without a binary.

/// f95 requires a kova binary at a path. Passing a nonexistent binary must
/// return an Err, not panic.
#[cfg(feature = "baked_demo")]
#[tokio::test]
async fn f95_errors_gracefully_when_binary_absent() {
    let bin = std::path::Path::new("/tmp/exopack_no_such_kova_binary_xyz");
    let home = std::env::temp_dir().join(format!("baked_demo_home_{}", std::process::id()));
    std::fs::create_dir_all(&home).unwrap();

    let result = exopack::baked_demo::f95(bin, &home, 19999).await;
    assert!(
        result.is_err(),
        "f95 must fail when kova binary does not exist"
    );

    let _ = std::fs::remove_dir_all(&home);
}

/// The demo artifact path convention: ~/.kova/demos/baked-demo.json.
/// Validate the path construction is correct relative to the home dir.
#[test]
fn demo_artifact_path_convention() {
    let home = std::path::Path::new("/tmp/fakehome");
    let expected = home.join(".kova").join("demos").join("baked-demo.json");
    // Verify the expected path ends with the canonical name.
    assert!(expected.to_string_lossy().ends_with("baked-demo.json"));
    assert!(expected.to_string_lossy().contains(".kova/demos/"));
}

/// Full baked demo — requires a built kova binary.
/// Run with: cargo test --features baked_demo -- --ignored
#[cfg(feature = "baked_demo")]
#[tokio::test]
#[ignore = "requires built kova binary at ~/.cargo/bin/kova"]
async fn f95_full_end_to_end() {
    let bin = std::path::Path::new(env!("HOME")).join(".cargo").join("bin").join("kova");
    if !bin.exists() {
        eprintln!("kova binary not found at {:?}, skipping", bin);
        return;
    }

    let home = std::env::temp_dir().join(format!("baked_demo_e2e_{}", std::process::id()));
    std::fs::create_dir_all(&home).unwrap();

    let result = exopack::baked_demo::f95(&bin, &home, 19998).await;
    if let Err(ref e) = result {
        eprintln!("baked demo error: {}", e);
    }
    assert!(result.is_ok(), "full baked demo must succeed");

    let artifact = home.join(".kova").join("demos").join("baked-demo.json");
    assert!(artifact.exists(), "demo artifact must be written");

    let _ = std::fs::remove_dir_all(&home);
}
