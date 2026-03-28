// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! exopack CLI: live-demo for -test binaries. Build and run with streaming output.

use std::path::PathBuf;
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_usage() {
    eprintln!("exopack {} — testing augmentation for Rust binaries", VERSION);
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  exopack live-demo <project_dir> [bin_name] [cargo_args...]");
    eprintln!("  exopack --help | -h");
    eprintln!("  exopack --version | -V");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  live-demo    Build and run a *-test binary with live output");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  exopack live-demo ./wowasticker --no-default-features --features tests");
    eprintln!("  exopack live-demo ./oakilydokily oakilydokily-test --features tests");
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
            println!("exopack {}", VERSION);
            exit(0);
        }
        "live-demo" => {}
        _ => {
            eprintln!("Unknown command: {}. Run 'exopack --help' for usage.", sub);
            exit(1);
        }
    }

    if args.len() < 3 {
        eprintln!("live-demo requires <project_dir>");
        eprintln!("  exopack live-demo <project_dir> [bin_name] [cargo_args...]");
        exit(1);
    }

    let project_dir = PathBuf::from(&args[2]);

    // Validate project dir exists before trying to parse Cargo.toml
    if !project_dir.join("Cargo.toml").exists() {
        eprintln!("Cargo.toml not found in {}", project_dir.display());
        exit(1);
    }

    let (bin_name, cargo_args): (String, Vec<&str>) = if args.len() >= 4 && !args[3].starts_with('-') {
        (args[3].clone(), args[4..].iter().map(|s| s.as_str()).collect())
    } else {
        match exopack::triple_sims::f63_discover_test_bin(&project_dir) {
            Some(b) => (b, args[3..].iter().map(|s| s.as_str()).collect()),
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
