// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! exopack CLI: live-demo, govdocs, SBOM. The binary IS the compliance artifact.

use std::path::PathBuf;
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

// Baked govdocs — compiled into the binary via include_str!
const GOVDOC_SBOM: &str = include_str!("../../govdocs/SBOM.md");
const GOVDOC_SECURITY: &str = include_str!("../../govdocs/SECURITY.md");
const GOVDOC_SSDF: &str = include_str!("../../govdocs/SSDF.md");
const GOVDOC_SUPPLY_CHAIN: &str = include_str!("../../govdocs/SUPPLY_CHAIN.md");
const GOVDOC_ACCESSIBILITY: &str = include_str!("../../govdocs/ACCESSIBILITY.md");
const GOVDOC_PRIVACY: &str = include_str!("../../govdocs/PRIVACY.md");
const GOVDOC_FIPS: &str = include_str!("../../govdocs/FIPS.md");
const GOVDOC_FEDRAMP: &str = include_str!("../../govdocs/FedRAMP_NOTES.md");
const GOVDOC_CMMC: &str = include_str!("../../govdocs/CMMC.md");
const GOVDOC_ITAR_EAR: &str = include_str!("../../govdocs/ITAR_EAR.md");
const GOVDOC_FEDERAL_USE: &str = include_str!("../../govdocs/FEDERAL_USE_CASES.md");

// Baked Cargo.toml for live SBOM generation
const CARGO_TOML: &str = include_str!("../../Cargo.toml");

fn print_usage() {
    eprintln!("{} {} — testing augmentation for Rust binaries", PKG_NAME, VERSION);
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  exopack live-demo <project_dir> [bin_name] [cargo_args...]");
    eprintln!("  exopack govdocs [topic]");
    eprintln!("  exopack --sbom");
    eprintln!("  exopack --help | -h");
    eprintln!("  exopack --version | -V");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  live-demo    Build and run a *-test binary with live output");
    eprintln!("  govdocs      Print federal compliance docs (baked into binary)");
    eprintln!();
    eprintln!("Govdocs topics:");
    eprintln!("  sbom, security, ssdf, supply-chain, accessibility, privacy,");
    eprintln!("  fips, fedramp, cmmc, itar-ear, federal-use-cases");
    eprintln!();
    eprintln!("Flags:");
    eprintln!("  --sbom       Machine-readable SPDX SBOM (for federal scanners)");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  exopack live-demo ./oakilydokily --features tests");
    eprintln!("  exopack govdocs sbom");
    eprintln!("  exopack --sbom > exopack.spdx");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        exit(1);
    }

    let sub = &args[1];
    match sub.as_str() {
        "--help" | "-h" | "help" => {
            print_usage();
            exit(0);
        }
        "--version" | "-V" => {
            println!("{} {}", PKG_NAME, VERSION);
            exit(0);
        }
        "--sbom" => {
            print_spdx_sbom();
            exit(0);
        }
        "govdocs" => {
            cmd_govdocs(&args[2..]);
            exit(0);
        }
        "live-demo" => cmd_live_demo(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}. Run 'exopack --help' for usage.", sub);
            exit(1);
        }
    }
}

// --- govdocs subcommand ---

fn cmd_govdocs(args: &[String]) {
    if args.is_empty() {
        // Print index
        println!("{} {} — Federal Compliance Documents", PKG_NAME, VERSION);
        println!();
        println!("Available topics:");
        println!("  sbom              Software Bill of Materials (EO 14028)");
        println!("  security          Security posture and attack surface");
        println!("  ssdf              NIST SP 800-218 SSDF mapping");
        println!("  supply-chain      Supply chain integrity");
        println!("  accessibility     Section 508 / WCAG compliance");
        println!("  privacy           Privacy impact assessment");
        println!("  fips              FIPS 140-2/3 status");
        println!("  fedramp           FedRAMP applicability notes");
        println!("  cmmc              CMMC Level 1-2 mapping");
        println!("  itar-ear          Export control classification");
        println!("  federal-use-cases Federal agency use cases");
        println!("  deps              Live dependency listing (from baked Cargo.toml)");
        println!();
        println!("Usage: exopack govdocs <topic>");
        println!("       exopack --sbom          (machine-readable SPDX)");
        return;
    }

    let topic = args[0].as_str();
    let doc = match topic {
        "sbom" => GOVDOC_SBOM,
        "security" => GOVDOC_SECURITY,
        "ssdf" => GOVDOC_SSDF,
        "supply-chain" => GOVDOC_SUPPLY_CHAIN,
        "accessibility" => GOVDOC_ACCESSIBILITY,
        "privacy" => GOVDOC_PRIVACY,
        "fips" => GOVDOC_FIPS,
        "fedramp" => GOVDOC_FEDRAMP,
        "cmmc" => GOVDOC_CMMC,
        "itar-ear" => GOVDOC_ITAR_EAR,
        "federal-use-cases" => GOVDOC_FEDERAL_USE,
        "deps" => {
            print_live_deps();
            return;
        }
        _ => {
            eprintln!("Unknown govdocs topic: {}. Run 'exopack govdocs' for list.", topic);
            exit(1);
        }
    };
    print!("{}", doc);
}

/// Parse the baked Cargo.toml and print live dependency info.
fn print_live_deps() {
    println!("{} {} — Live Dependency Listing", PKG_NAME, VERSION);
    println!("Source: Cargo.toml baked into binary at compile time");
    println!();

    let mut in_deps = false;
    let mut deps: Vec<(String, String, bool)> = Vec::new();

    for line in CARGO_TOML.lines() {
        let trimmed = line.trim();
        if trimmed == "[dependencies]" {
            in_deps = true;
            continue;
        }
        if trimmed.starts_with('[') && in_deps {
            break;
        }
        if in_deps && trimmed.contains('=') && !trimmed.starts_with('#') {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let rest = parts[1].trim();
                let version = if rest.starts_with('"') {
                    rest.trim_matches('"').to_string()
                } else if rest.contains("version") {
                    rest.split('"')
                        .nth(1)
                        .unwrap_or("?")
                        .to_string()
                } else {
                    "?".to_string()
                };
                let optional = rest.contains("optional = true");
                deps.push((name, version, optional));
            }
        }
    }

    println!("{:<20} {:<10} {}", "CRATE", "VERSION", "OPTIONAL");
    println!("{:<20} {:<10} {}", "-----", "-------", "--------");
    for (name, ver, opt) in &deps {
        println!("{:<20} {:<10} {}", name, ver, if *opt { "yes" } else { "no" });
    }
    println!();
    println!("Total: {} direct dependencies ({} optional)",
        deps.len(), deps.iter().filter(|d| d.2).count());
}

/// Print SPDX 2.3 format SBOM — machine-readable for federal scanners.
fn print_spdx_sbom() {
    println!("SPDXVersion: SPDX-2.3");
    println!("DataLicense: CC0-1.0");
    println!("SPDXID: SPDXRef-DOCUMENT");
    println!("DocumentName: {}-{}", PKG_NAME, VERSION);
    println!("DocumentNamespace: https://github.com/cochranblock/{}/releases/tag/v{}", PKG_NAME, VERSION);
    println!("Creator: Tool: exopack-{}", VERSION);
    println!();

    // Package info
    println!("PackageName: {}", PKG_NAME);
    println!("SPDXID: SPDXRef-Package");
    println!("PackageVersion: {}", VERSION);
    println!("PackageDownloadLocation: https://github.com/cochranblock/{}", PKG_NAME);
    println!("PackageLicenseConcluded: Unlicense");
    println!("PackageLicenseDeclared: Unlicense");
    println!("PackageCopyrightText: NOASSERTION");
    println!("PackageSupplier: Organization: The Cochran Block");
    println!();

    // Parse deps from baked Cargo.toml
    let mut in_deps = false;
    for line in CARGO_TOML.lines() {
        let trimmed = line.trim();
        if trimmed == "[dependencies]" {
            in_deps = true;
            continue;
        }
        if trimmed.starts_with('[') && in_deps {
            break;
        }
        if in_deps && trimmed.contains('=') && !trimmed.starts_with('#') {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                let name = parts[0].trim();
                let rest = parts[1].trim();
                let version = if rest.starts_with('"') {
                    rest.trim_matches('"')
                } else if rest.contains("version") {
                    rest.split('"').nth(1).unwrap_or("?")
                } else {
                    "?"
                };
                let spdx_id = format!("SPDXRef-Crate-{}", name.replace('-', "_"));
                println!("PackageName: {}", name);
                println!("SPDXID: {}", spdx_id);
                println!("PackageVersion: {}", version);
                println!("PackageDownloadLocation: https://crates.io/crates/{}/{}", name, version);
                println!("ExternalRef: PACKAGE-MANAGER purl pkg:cargo/{}@{}", name, version);
                println!("PackageLicenseConcluded: NOASSERTION");
                println!("Relationship: SPDXRef-Package DEPENDS_ON {}", spdx_id);
                println!();
            }
        }
    }
}

// --- live-demo subcommand ---

fn cmd_live_demo(args: &[String]) {
    if args.is_empty() {
        eprintln!("live-demo requires <project_dir>");
        eprintln!("  exopack live-demo <project_dir> [bin_name] [cargo_args...]");
        exit(1);
    }

    let project_dir = PathBuf::from(&args[0]);

    if !project_dir.join("Cargo.toml").exists() {
        eprintln!("Cargo.toml not found in {}", project_dir.display());
        exit(1);
    }

    let (bin_name, cargo_args): (String, Vec<&str>) = if args.len() >= 2 && !args[1].starts_with('-') {
        (args[1].clone(), args[2..].iter().map(|s| s.as_str()).collect())
    } else {
        match exopack::triple_sims::f63_discover_test_bin(&project_dir) {
            Some(b) => (b, args[1..].iter().map(|s| s.as_str()).collect()),
            None => {
                eprintln!("No *-test binary found in {}. Specify bin_name explicitly.", project_dir.display());
                eprintln!("  exopack live-demo {} <bin_name> [cargo_args...]", project_dir.display());
                exit(1);
            }
        }
    };

    println!("exopack live-demo: building and running {} in {}...", bin_name, project_dir.display());
    match exopack::triple_sims::f62_live_demo(&project_dir, bin_name.as_str(), &cargo_args) {
        Ok(status) => exit(status.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("exopack live-demo: {}", e);
            exit(1);
        }
    }
}
