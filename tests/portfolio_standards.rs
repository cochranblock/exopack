// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Integration test: run standards check against all cochranblock projects.

#[cfg(feature = "standards_check")]
#[test]
fn portfolio_standards_gate() {
    use std::path::Path;

    let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/mcochran".to_string());
    let base = Path::new(&home);

    let projects: Vec<std::path::PathBuf> = [
        "exopack",
        "kova",
        "cochranblock",
        "oakilydokily",
        "approuter",
        "rogue-repo",
        "pixel-forge",
        "whyyoulying",
        "wowasticker",
        "ronin-sites",
    ]
    .iter()
    .map(|name| base.join(name))
    .filter(|p| p.join("Cargo.toml").exists())
    .collect();

    let project_refs: Vec<&Path> = projects.iter().map(|p| p.as_ref()).collect();
    let report = exopack::standards_check::f116(&project_refs);
    report.print_table();

    // Print detail for failures
    println!("\n--- FAILURES ---");
    for proj in &report.s86 {
        for check in &proj.s85 {
            if !check.s81 {
                println!("  {} / {}: {}", proj.s83, check.s80, check.s82);
            }
        }
    }

    // Report summary
    let total_checks: usize = report.s86.iter().map(|p| p.total()).sum();
    let total_passed: usize = report.s86.iter().map(|p| p.passed()).sum();
    println!("\nPORTFOLIO: {}/{} checks passed across {} projects",
        total_passed, total_checks, report.s86.len());
}
