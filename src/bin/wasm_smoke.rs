// © 2026 Cochran Block. All rights reserved.
//
// E2E smoke test for the dndaimodel WASM page.
//
// Loads the page, waits for `init()` + WASM instantiation, clicks the Forge button,
// waits for the result, asserts it's non-empty. Captures console errors throughout.
// Saves a screenshot of the rendered page on success.
//
// Usage:
//   cargo run --release --features devtools --bin wasm-smoke -- \
//     --url http://192.168.1.52:7720/ --out /tmp/dndaimodel-smoke.png

use std::path::PathBuf;
use std::time::Duration;

use chromiumoxide::Browser;
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use futures::StreamExt;

#[derive(Debug)]
struct Args {
    url: String,
    out: PathBuf,
}

fn parse_args() -> Args {
    let mut url = "http://127.0.0.1:7720/".to_string();
    let mut out = PathBuf::from("/tmp/dndaimodel-smoke.png");
    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--url" => url = iter.next().unwrap_or(url),
            "--out" => out = iter.next().map(PathBuf::from).unwrap_or(out),
            _ => {}
        }
    }
    Args { url, out }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), String> {
    let args = parse_args();
    eprintln!("smoke: url={}  out={}", args.url, args.out.display());

    let config = exopack::devtools::browser_config()
        .await
        .map_err(|e| format!("browser_config: {e}"))?;
    let (mut browser, mut handler) = Browser::launch(config)
        .await
        .map_err(|e| format!("Browser::launch: {e}"))?;
    let handler_task = tokio::spawn(async move { while handler.next().await.is_some() {} });

    let result = run_smoke(&browser, &args).await;
    let _ = browser.close().await;
    handler_task.abort();

    match &result {
        Ok(report) => {
            eprintln!("\n=== SMOKE PASS ===");
            eprintln!("{report}");
        }
        Err(e) => {
            eprintln!("\n=== SMOKE FAIL ===");
            eprintln!("{e}");
        }
    }
    result.map(|_| ())
}

async fn run_smoke(browser: &Browser, args: &Args) -> Result<String, String> {
    let page = browser
        .new_page("about:blank")
        .await
        .map_err(|e| format!("new_page: {e}"))?;

    // collect console messages
    let mut console = page
        .event_listener::<chromiumoxide::cdp::browser_protocol::log::EventEntryAdded>()
        .await
        .map_err(|e| format!("listen log: {e}"))?;
    let console_log: std::sync::Arc<std::sync::Mutex<Vec<String>>> = Default::default();
    {
        let log = console_log.clone();
        tokio::spawn(async move {
            while let Some(entry) = console.next().await {
                if entry.entry.level == chromiumoxide::cdp::browser_protocol::log::LogEntryLevel::Error {
                    if let Ok(mut g) = log.lock() {
                        g.push(format!("[{:?}] {}", entry.entry.source, entry.entry.text));
                    }
                }
            }
        });
    }

    page.goto(&args.url)
        .await
        .map_err(|e| format!("goto: {e}"))?;
    page.wait_for_navigation()
        .await
        .map_err(|e| format!("wait_for_navigation: {e}"))?;

    // Give the module loader a moment to import + start init().
    tokio::time::sleep(Duration::from_millis(800)).await;

    // Find the Forge button.
    let title: String = page
        .evaluate("document.title")
        .await
        .map_err(|e| format!("evaluate title: {e}"))?
        .into_value()
        .map_err(|e| format!("title value: {e}"))?;
    eprintln!("page title: {title}");

    // Click Forge — this triggers ensureLoaded() (fetch wasm + model + sig) and
    // generate_spell_name().
    page.evaluate("document.getElementById('gen-go').click()")
        .await
        .map_err(|e| format!("click: {e}"))?;

    // Poll for output for up to 20s. ensureLoaded() includes wasm fetch + safetensors
    // download + 132K-param model load; cold path takes a couple of seconds.
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(20);
    let mut last_status = String::new();
    let result_text;
    loop {
        let status: String = page
            .evaluate("document.getElementById('gen-status').textContent")
            .await
            .map_err(|e| format!("read status: {e}"))?
            .into_value()
            .map_err(|e| format!("status value: {e}"))?;
        if status != last_status {
            eprintln!("  status: {status}");
            last_status = status.clone();
        }
        let out: String = page
            .evaluate("document.getElementById('gen-out').textContent")
            .await
            .map_err(|e| format!("read out: {e}"))?
            .into_value()
            .map_err(|e| format!("out value: {e}"))?;
        if !out.trim().is_empty() {
            result_text = out;
            break;
        }
        if start.elapsed() > timeout {
            return Err(format!(
                "timed out waiting for output. last status: {last_status}"
            ));
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    // Screenshot for proof.
    let screenshot = page
        .save_screenshot(
            chromiumoxide::page::ScreenshotParams::builder()
                .format(CaptureScreenshotFormat::Png)
                .full_page(true)
                .build(),
            &args.out,
        )
        .await
        .map_err(|e| format!("screenshot: {e}"))?;

    let errors = console_log.lock().map(|g| g.clone()).unwrap_or_default();
    let mut report = String::new();
    report.push_str(&format!("page title:   {title}\n"));
    report.push_str(&format!("forge output: {result_text:?}\n"));
    report.push_str(&format!(
        "screenshot:   {} ({} bytes)\n",
        args.out.display(),
        screenshot.len()
    ));
    report.push_str(&format!("console errs: {} (must be 0)\n", errors.len()));
    if !errors.is_empty() {
        for e in &errors {
            report.push_str(&format!("  - {e}\n"));
        }
        return Err(report);
    }
    if result_text.trim().is_empty() {
        return Err(format!("{report}\nFAIL: empty result text"));
    }
    Ok(report)
}
