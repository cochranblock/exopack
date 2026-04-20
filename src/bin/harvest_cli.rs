// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! exopack-harvest CLI — batch Gemini sprite generation via headless Chrome CDP.
//!
//! Usage:
//!   exopack-harvest --class skeleton --count 5
//!   exopack-harvest --class skeleton,zombie,dwarf,goblin --count 3
//!   exopack-harvest --class skeleton --count 1 --launch  (auto-launch debug Chrome)
//!   exopack-harvest --list-gaps                          (show classes under 200 sprites)

use std::path::PathBuf;

fn print_usage() {
    eprintln!("exopack-harvest — batch Gemini sprite sheet generation");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  exopack-harvest --class <names> --count <n> [--output <dir>] [--port <n>] [--launch] [--style <ref>]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --class <names>   Comma-separated class names (e.g. skeleton,zombie,dwarf)");
    eprintln!("  --count <n>       Sheets per class (each sheet = 30 sprites)");
    eprintln!("  --output <dir>    Output directory [default: for_human_review]");
    eprintln!("  --port <n>        Chrome debug port [default: 9222]");
    eprintln!("  --launch          Auto-launch headless Chrome with your profile");
    eprintln!("  --style <ref>     Art style reference [default: Dungeon Crawl Stone Soup]");
    eprintln!();
    eprintln!("Prerequisites:");
    eprintln!("  Chrome running with: --remote-debugging-port=9222");
    eprintln!("  Logged into gemini.google.com");
    eprintln!("  Or use --launch to auto-start headless Chrome with your cookies");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_usage();
        std::process::exit(if args.len() < 2 { 1 } else { 0 });
    }

    let mut classes: Vec<String> = Vec::new();
    let mut count: usize = 1;
    let mut output = PathBuf::from("for_human_review");
    let mut port: u16 = 9222;
    let mut launch = false;
    let mut style = "Dungeon Crawl Stone Soup".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--class" => {
                i += 1;
                if i < args.len() {
                    classes = args[i].split(',').map(|s| s.trim().to_string()).collect();
                }
            }
            "--count" => {
                i += 1;
                if i < args.len() {
                    count = args[i].parse().unwrap_or(1);
                }
            }
            "--output" => {
                i += 1;
                if i < args.len() {
                    output = PathBuf::from(&args[i]);
                }
            }
            "--port" => {
                i += 1;
                if i < args.len() {
                    port = args[i].parse().unwrap_or(9222);
                }
            }
            "--launch" => launch = true,
            "--style" => {
                i += 1;
                if i < args.len() {
                    style = args[i].clone();
                }
            }
            other => {
                eprintln!("unknown arg: {}", other);
                print_usage();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    if classes.is_empty() {
        eprintln!("error: --class required");
        print_usage();
        std::process::exit(1);
    }

    // Launch headless Chrome if requested
    if launch {
        eprintln!("[harvest] launching headless Chrome on port {}...", port);
        match exopack::harvest::launch_debug_chrome(port).await {
            Ok(url) => eprintln!("[harvest] Chrome ready at {}", url),
            Err(e) => {
                eprintln!("[harvest] launch failed: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Find Gemini tab
    eprintln!("[harvest] connecting to Chrome on port {}...", port);
    let ws_url = match exopack::harvest::find_gemini_ws(port).await {
        Ok(ws) => {
            eprintln!("[harvest] found Gemini tab");
            ws
        }
        Err(e) => {
            eprintln!("[harvest] {}", e);
            std::process::exit(1);
        }
    };

    // Build batch list
    let batch: Vec<(&str, usize)> = classes.iter().map(|c| (c.as_str(), count)).collect();
    let total = classes.len() * count;
    eprintln!(
        "[harvest] generating {} sheets across {} classes ({} sprites)",
        total,
        classes.len(),
        total * 30
    );

    // Run batch harvest
    std::fs::create_dir_all(&output).ok();
    match exopack::harvest::harvest_batch(&ws_url, &output, &batch, &style).await {
        Ok(paths) => {
            eprintln!("[harvest] done — {} sheets saved to {}", paths.len(), output.display());
            for p in &paths {
                println!("{}", p.display());
            }
        }
        Err(e) => {
            eprintln!("[harvest] batch failed: {}", e);
            std::process::exit(1);
        }
    }
}
