// Unlicense — public domain — cochranblock.org
//! exopack CLI: live-demo, govdocs, SBOM. The binary IS the compliance artifact.

#[cfg(feature = "triple_sims")]
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
    eprintln!(
        "{} {} — testing augmentation for Rust binaries",
        PKG_NAME, VERSION
    );
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  exopack live-demo <project_dir> [bin_name] [cargo_args...]");
    eprintln!("  exopack standards <project_dir> [--json]");
    eprintln!("  exopack baselines accept <project>");
    eprintln!("  exopack screenshot <url> <out.png>");
    eprintln!("  exopack compare <a.png> <b.png> [--tolerance N] [--threshold P]");
    eprintln!("  exopack ats-fixture <vendor> [--dynamic-ids] [--late-hydration MS] [--rebuild-on-focus] [--out FILE]");
    eprintln!("  exopack govdocs [topic]");
    eprintln!("  exopack --sbom");
    eprintln!("  exopack --help | -h");
    eprintln!("  exopack --version | -V");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  live-demo    Build and run a *-test binary with live output (feature: triple_sims)");
    eprintln!("  standards    Run the 14-point Rust standards gate against a project (feature: standards_check)");
    eprintln!("  baselines    Manage Sim 4 baselines — `accept` promotes pending → trusted (feature: screenshot)");
    eprintln!("  screenshot   Capture a single page via headless Chromium (features: screenshot,devtools)");
    eprintln!("  compare      Pixel-diff two PNGs, exit nonzero on mismatch (feature: screenshot)");
    eprintln!("  ats-fixture  Emit a self-contained mock ATS application page HTML (feature: ats_fixtures)");
    eprintln!("                 vendors: greenhouse, lever, workday, icims, ashby");
    eprintln!("  govdocs      Print federal compliance docs (baked into binary)");
    eprintln!();
    eprintln!("Govdocs topics:");
    eprintln!("  sbom, security, ssdf, supply-chain, accessibility, privacy,");
    eprintln!("  fips, fedramp, cmmc, itar-ear, federal-use-cases");
    eprintln!();
    eprintln!("Flags:");
    eprintln!("  --sbom       Machine-readable SPDX SBOM (for federal scanners)");
    eprintln!();
    eprintln!("Build with `--features cli` for the full subcommand set, or pick the");
    eprintln!("specific feature listed beside each command above.");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  exopack live-demo ./oakilydokily --features tests");
    eprintln!("  exopack standards . --json");
    eprintln!("  exopack baselines accept myapp");
    eprintln!("  exopack compare current.png baseline.png");
    eprintln!("  exopack ats-fixture workday --dynamic-ids --late-hydration 500 > workday.html");
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
        "standards" => cmd_standards(&args[2..]),
        "baselines" => cmd_baselines(&args[2..]),
        "screenshot" => cmd_screenshot(&args[2..]),
        "compare" => cmd_compare(&args[2..]),
        "ats-fixture" => cmd_ats_fixture(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}. Run 'exopack --help' for usage.", sub);
            exit(1);
        }
    }
}

// --- standards subcommand (feature: standards_check) ---

#[cfg(feature = "standards_check")]
fn cmd_standards(args: &[String]) {
    if args.is_empty() {
        eprintln!("standards requires <project_dir>");
        eprintln!("  exopack standards <project_dir> [--json]");
        exit(1);
    }
    let project_dir = PathBuf::from(&args[0]);
    let json_output = args.iter().any(|a| a == "--json");

    let report = exopack::standards_check::f101(&project_dir);
    let failed = report.failed();

    if json_output {
        // Hand-rolled JSON to avoid a serde_json dep at the binary level.
        print!("{{");
        print!("\"project\":{},", json_str(&report.s83));
        print!("\"path\":{},", json_str(&report.s84.display().to_string()));
        print!("\"total\":{},", report.total());
        print!("\"passed\":{},", report.passed());
        print!("\"failed\":{},", report.failed());
        print!("\"checks\":[");
        for (i, c) in report.s85.iter().enumerate() {
            if i > 0 { print!(","); }
            print!(
                "{{\"name\":{},\"passed\":{},\"detail\":{}}}",
                json_str(c.s80),
                c.s81,
                json_str(&c.s82)
            );
        }
        println!("]}}");
    } else {
        println!("STANDARDS CHECK: {} ({}/{} passed)", report.s83, report.passed(), report.total());
        for c in &report.s85 {
            let icon = if c.s81 { "PASS" } else { "FAIL" };
            println!("  [{}] {:<18} {}", icon, c.s80, c.s82);
        }
    }
    exit(if failed == 0 { 0 } else { 1 });
}

#[cfg(not(feature = "standards_check"))]
fn cmd_standards(_args: &[String]) {
    eprintln!("`standards` requires building with --features standards_check (or --features cli).");
    exit(2);
}

#[cfg(feature = "standards_check")]
fn json_str(s: &str) -> String {
    // Minimal JSON string escape — enough for project names, paths, and check details.
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

// --- baselines subcommand (feature: screenshot) ---

#[cfg(feature = "screenshot")]
fn cmd_baselines(args: &[String]) {
    let usage = || {
        eprintln!("baselines accept <project>");
        eprintln!("  Promotes baselines_pending/* to baselines/* for the named project.");
    };
    if args.len() < 2 || args[0] != "accept" {
        usage();
        exit(1);
    }
    let project = &args[1];
    match exopack::screenshot::accept_pending_baselines(project) {
        Ok(0) => {
            eprintln!("baselines accept: nothing pending for project {}", project);
            exit(0);
        }
        Ok(n) => {
            println!("baselines accept: promoted {} file(s)", n);
            exit(0);
        }
        Err(e) => {
            eprintln!("baselines accept: {}", e);
            exit(1);
        }
    }
}

#[cfg(not(feature = "screenshot"))]
fn cmd_baselines(_args: &[String]) {
    eprintln!("`baselines` requires building with --features screenshot.");
    exit(2);
}

// --- screenshot subcommand (features: screenshot + devtools for real captures) ---

#[cfg(all(feature = "screenshot", feature = "devtools"))]
fn cmd_screenshot(args: &[String]) {
    if args.len() < 2 {
        eprintln!("screenshot <url> <out.png>");
        exit(1);
    }
    let url = &args[0];
    let out = std::path::PathBuf::from(&args[1]);

    // Minimal block_on for the async devtools call.
    let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("screenshot: tokio runtime: {}", e);
            exit(1);
        }
    };
    let parent = out.parent().unwrap_or(std::path::Path::new("."));
    if let Err(e) = std::fs::create_dir_all(parent) {
        eprintln!("screenshot: mkdir {}: {}", parent.display(), e);
        exit(1);
    }
    let file_stem = out
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "screenshot".to_string());

    let result = rt.block_on(async {
        exopack::devtools::f75(url, &[(file_stem.as_str(), "")], parent).await
    });
    match result {
        Ok(true) => {
            println!("screenshot: saved {}", out.display());
            exit(0);
        }
        Ok(false) => {
            eprintln!("screenshot: capture reported failure");
            exit(1);
        }
        Err(e) => {
            eprintln!("screenshot: {}", e);
            exit(1);
        }
    }
}

#[cfg(not(all(feature = "screenshot", feature = "devtools")))]
fn cmd_screenshot(_args: &[String]) {
    eprintln!("`screenshot` requires building with --features \"screenshot devtools\".");
    exit(2);
}

// --- compare subcommand (feature: screenshot) ---

#[cfg(feature = "screenshot")]
fn cmd_compare(args: &[String]) {
    if args.len() < 2 {
        eprintln!("compare <a.png> <b.png> [--tolerance N] [--threshold P]");
        exit(1);
    }
    let a = std::path::PathBuf::from(&args[0]);
    let b = std::path::PathBuf::from(&args[1]);
    let mut tolerance: u8 = 10;
    let mut threshold: f64 = 1.0;
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--tolerance" => {
                i += 1;
                if let Some(v) = args.get(i).and_then(|s| s.parse().ok()) { tolerance = v; }
            }
            "--threshold" => {
                i += 1;
                if let Some(v) = args.get(i).and_then(|s| s.parse().ok()) { threshold = v; }
            }
            other => {
                eprintln!("compare: unknown arg {}", other);
                exit(1);
            }
        }
        i += 1;
    }
    match exopack::screenshot::compare_screenshots(&a, &b, tolerance, threshold) {
        Ok(r) => {
            println!(
                "compare: {} vs {} — {:.2}% diff ({}/{} pixels)",
                a.display(), b.display(), r.diff_pct, r.diff_pixels, r.total_pixels
            );
            exit(if r.matches { 0 } else { 1 });
        }
        Err(e) => {
            eprintln!("compare: {}", e);
            exit(1);
        }
    }
}

#[cfg(not(feature = "screenshot"))]
fn cmd_compare(_args: &[String]) {
    eprintln!("`compare` requires building with --features screenshot.");
    exit(2);
}

// --- ats-fixture subcommand (feature: ats_fixtures) ---

#[cfg(feature = "ats_fixtures")]
fn cmd_ats_fixture(args: &[String]) {
    use exopack::ats_fixtures::{render, AtsVendor, FixtureOpts};

    if args.is_empty() {
        eprintln!("ats-fixture <vendor> [--dynamic-ids] [--late-hydration MS] [--rebuild-on-focus] [--out FILE]");
        eprintln!("  vendor: greenhouse | lever | workday | icims | ashby");
        exit(1);
    }

    let vendor = match args[0].to_ascii_lowercase().as_str() {
        "greenhouse" => AtsVendor::Greenhouse,
        "lever" => AtsVendor::Lever,
        "workday" => AtsVendor::Workday,
        "icims" => AtsVendor::Icims,
        "ashby" => AtsVendor::Ashby,
        other => {
            eprintln!("ats-fixture: unknown vendor `{}`. Try greenhouse|lever|workday|icims|ashby.", other);
            exit(1);
        }
    };

    let mut opts = FixtureOpts::default();
    let mut out_path: Option<std::path::PathBuf> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--dynamic-ids" => opts.dynamic_ids = true,
            "--rebuild-on-focus" => opts.rebuild_on_focus = true,
            "--late-hydration" => {
                i += 1;
                opts.late_hydration_ms = args.get(i).and_then(|s| s.parse().ok());
                if opts.late_hydration_ms.is_none() {
                    eprintln!("ats-fixture: --late-hydration needs a milliseconds integer");
                    exit(1);
                }
            }
            "--out" => {
                i += 1;
                out_path = args.get(i).map(std::path::PathBuf::from);
                if out_path.is_none() {
                    eprintln!("ats-fixture: --out needs a path");
                    exit(1);
                }
            }
            other => {
                eprintln!("ats-fixture: unknown arg `{}`", other);
                exit(1);
            }
        }
        i += 1;
    }

    let html = render(vendor, &opts);
    match out_path {
        Some(path) => match std::fs::write(&path, &html) {
            Ok(_) => {
                eprintln!("ats-fixture: wrote {} ({} bytes)", path.display(), html.len());
            }
            Err(e) => {
                eprintln!("ats-fixture: write {}: {}", path.display(), e);
                exit(1);
            }
        },
        None => {
            // Stream to stdout for piping into a file or another tool.
            print!("{}", html);
        }
    }
}

#[cfg(not(feature = "ats_fixtures"))]
fn cmd_ats_fixture(_args: &[String]) {
    eprintln!("`ats-fixture` requires building with --features ats_fixtures.");
    exit(2);
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
            eprintln!(
                "Unknown govdocs topic: {}. Run 'exopack govdocs' for list.",
                topic
            );
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
                    rest.split('"').nth(1).unwrap_or("?").to_string()
                } else {
                    "?".to_string()
                };
                let optional = rest.contains("optional = true");
                deps.push((name, version, optional));
            }
        }
    }

    println!("{:<20} {:<10} OPTIONAL", "CRATE", "VERSION");
    println!("{:<20} {:<10} --------", "-----", "-------");
    for (name, ver, opt) in &deps {
        println!(
            "{:<20} {:<10} {}",
            name,
            ver,
            if *opt { "yes" } else { "no" }
        );
    }
    println!();
    println!(
        "Total: {} direct dependencies ({} optional)",
        deps.len(),
        deps.iter().filter(|d| d.2).count()
    );
}

/// Print SPDX 2.3 format SBOM — machine-readable for federal scanners.
fn print_spdx_sbom() {
    println!("SPDXVersion: SPDX-2.3");
    println!("DataLicense: CC0-1.0");
    println!("SPDXID: SPDXRef-DOCUMENT");
    println!("DocumentName: {}-{}", PKG_NAME, VERSION);
    println!(
        "DocumentNamespace: https://github.com/cochranblock/{}/releases/tag/v{}",
        PKG_NAME, VERSION
    );
    println!("Creator: Tool: exopack-{}", VERSION);
    println!();

    // Package info
    println!("PackageName: {}", PKG_NAME);
    println!("SPDXID: SPDXRef-Package");
    println!("PackageVersion: {}", VERSION);
    println!(
        "PackageDownloadLocation: https://github.com/cochranblock/{}",
        PKG_NAME
    );
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
                println!(
                    "PackageDownloadLocation: https://crates.io/crates/{}/{}",
                    name, version
                );
                println!(
                    "ExternalRef: PACKAGE-MANAGER purl pkg:cargo/{}@{}",
                    name, version
                );
                println!("PackageLicenseConcluded: NOASSERTION");
                println!("Relationship: SPDXRef-Package DEPENDS_ON {}", spdx_id);
                println!();
            }
        }
    }
}

// --- live-demo subcommand (feature: triple_sims) ---

#[cfg(feature = "triple_sims")]
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

    let (bin_name, cargo_args): (String, Vec<&str>) =
        if args.len() >= 2 && !args[1].starts_with('-') {
            (
                args[1].clone(),
                args[2..].iter().map(|s| s.as_str()).collect(),
            )
        } else {
            match exopack::triple_sims::discover_test_bin(&project_dir) {
                Some(b) => (b, args[1..].iter().map(|s| s.as_str()).collect()),
                None => {
                    eprintln!(
                        "No *-test binary found in {}. Specify bin_name explicitly.",
                        project_dir.display()
                    );
                    eprintln!(
                        "  exopack live-demo {} <bin_name> [cargo_args...]",
                        project_dir.display()
                    );
                    exit(1);
                }
            }
        };

    println!(
        "exopack live-demo: building and running {} in {}...",
        bin_name,
        project_dir.display()
    );
    match exopack::triple_sims::live_demo(&project_dir, bin_name.as_str(), &cargo_args) {
        Ok(status) => exit(status.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("exopack live-demo: {}", e);
            exit(1);
        }
    }
}

#[cfg(not(feature = "triple_sims"))]
fn cmd_live_demo(_args: &[String]) {
    eprintln!("`live-demo` requires building with --features triple_sims.");
    exit(2);
}
