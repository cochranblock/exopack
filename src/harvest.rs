// Unlicense — public domain — cochranblock.org
//! Harvest: connect to running Chrome via CDP, automate Gemini sprite generation.
//!
//! Connects to Chrome on --remote-debugging-port=9222, navigates Gemini,
//! sends sprite sheet prompts, waits for image generation, extracts via
//! canvas blob→base64, saves to for_human_review/.
//!
//! Usage: Chrome must be running with --remote-debugging-port=9222 and
//! logged into Gemini. This module does NOT launch Chrome — it connects
//! to the existing authenticated session.

use std::path::{Path, PathBuf};
use std::time::Duration;

/// Gemini sprite harvest configuration.
pub struct HarvestConfig {
    /// Chrome DevTools websocket URL (from http://localhost:9222/json)
    pub ws_url: String,
    /// Output directory for generated sprite sheets
    pub output_dir: PathBuf,
    /// Sprite class name (e.g. "skeleton", "knight", "zombie")
    pub class_name: String,
    /// Number of sheets to generate per class
    pub sheet_count: usize,
    /// Art style reference (e.g. "Dungeon Crawl Stone Soup")
    pub style: String,
}

impl Default for HarvestConfig {
    fn default() -> Self {
        Self {
            ws_url: String::new(),
            output_dir: PathBuf::from("for_human_review"),
            class_name: "skeleton".into(),
            sheet_count: 1,
            style: "Dungeon Crawl Stone Soup".into(),
        }
    }
}

/// Build the prompt for a 5×6 sprite sheet.
pub fn build_prompt(class: &str, style: &str) -> String {
    format!(
        "Generate a 5 columns by 6 rows sprite sheet of 32x32 pixel art {} on a white background. \
         Each sprite should be unique - different poses, weapons, armor pieces. \
         Classic retro RPG style like {}. \
         No text, no labels, just the sprites on a clean white grid.",
        class, style
    )
}

/// Discover the websocket URL for the Gemini tab from the CDP endpoint.
pub async fn find_gemini_ws(debug_port: u16) -> Result<String, String> {
    let url = format!("http://localhost:{}/json", debug_port);
    let body = reqwest::get(&url)
        .await
        .map_err(|e| format!("CDP connect failed: {}. Is Chrome running with --remote-debugging-port={}?", e, debug_port))?
        .text()
        .await
        .map_err(|e| format!("CDP read: {}", e))?;

    let tabs: Vec<serde_json::Value> = serde_json::from_str(&body)
        .map_err(|e| format!("CDP parse: {}", e))?;

    for tab in &tabs {
        let tab_url = tab.get("url").and_then(|v| v.as_str()).unwrap_or("");
        if tab_url.contains("gemini.google.com/app") {
            if let Some(ws) = tab.get("webSocketDebuggerUrl").and_then(|v| v.as_str()) {
                return Ok(ws.to_string());
            }
        }
    }
    Err("no Gemini tab found — open gemini.google.com/app in Chrome first".into())
}

/// Navigate to a fresh Gemini chat.
pub async fn navigate_fresh(ws_url: &str) -> Result<(), String> {
    // Uses CDP Page.navigate to open a fresh Gemini session
    let msg = serde_json::json!({
        "id": 1,
        "method": "Page.navigate",
        "params": {"url": "https://gemini.google.com/app"}
    });
    cdp_send(ws_url, &msg).await?;
    tokio::time::sleep(Duration::from_secs(4)).await;
    Ok(())
}

/// Type a prompt into Gemini's input box.
pub async fn type_prompt(ws_url: &str, prompt: &str) -> Result<(), String> {
    let escaped = prompt.replace('\\', "\\\\").replace('`', "\\`").replace('"', "\\\"");
    let js = format!(
        r#"(() => {{
            let el = document.querySelector('rich-textarea [contenteditable="true"]')
                  || document.querySelector('[contenteditable="true"]')
                  || document.querySelector('textarea');
            if (!el) return 'NO_INPUT';
            el.focus();
            el.textContent = "{}";
            el.dispatchEvent(new Event('input', {{bubbles: true}}));
            return 'TYPED';
        }})()"#,
        escaped
    );
    let result = cdp_eval(ws_url, &js).await?;
    if result != "TYPED" {
        return Err(format!("type failed: {}", result));
    }
    Ok(())
}

/// Click the send button.
pub async fn click_send(ws_url: &str) -> Result<(), String> {
    let js = r#"(() => {
        let btn = document.querySelector('[aria-label="Send message"]')
               || document.querySelector('button[data-at="send"]')
               || document.querySelector('.send-button')
               || document.querySelector('mat-icon[data-mat-icon-name="send"]');
        if (btn) { (btn.closest('button') || btn).click(); return 'CLICKED'; }
        let el = document.querySelector('[contenteditable="true"]');
        if (el) {
            el.dispatchEvent(new KeyboardEvent('keydown', {key: 'Enter', code: 'Enter', bubbles: true}));
            return 'ENTER';
        }
        return 'NO_BUTTON';
    })()"#;
    let result = cdp_eval(ws_url, js).await?;
    if result == "NO_BUTTON" {
        return Err("no send button found".into());
    }
    Ok(())
}

/// Wait for image generation to complete. Polls every 3s, max 90s.
pub async fn wait_for_image(ws_url: &str) -> Result<(), String> {
    for _ in 0..30 {
        let js = r#"(() => {
            let creating = document.body.innerText.includes('Creating your image');
            let imgs = [...document.querySelectorAll('img')].filter(i => i.src.startsWith('blob:') && i.naturalWidth > 100);
            return JSON.stringify({creating: creating, count: imgs.length});
        })()"#;
        let val = cdp_eval(ws_url, js).await?;
        if let Ok(status) = serde_json::from_str::<serde_json::Value>(&val) {
            let creating = status.get("creating").and_then(|v| v.as_bool()).unwrap_or(true);
            let count = status.get("count").and_then(|v| v.as_u64()).unwrap_or(0);
            if !creating && count > 0 {
                return Ok(());
            }
        }
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
    Err("timeout waiting for image (90s)".into())
}

/// Extract the generated image via canvas and save to disk.
pub async fn extract_image(ws_url: &str, output_path: &Path) -> Result<usize, String> {
    let js = r#"(async () => {
        let img = [...document.querySelectorAll('img')].find(i => i.src.startsWith('blob:') && i.naturalWidth > 100);
        if (!img) return 'NO_IMG';
        await new Promise(r => { if (img.complete) r(); else img.onload = r; });
        let canvas = document.createElement('canvas');
        canvas.width = img.naturalWidth;
        canvas.height = img.naturalHeight;
        canvas.getContext('2d').drawImage(img, 0, 0);
        return canvas.toDataURL('image/png');
    })()"#;

    let data_url = cdp_eval_async(ws_url, js).await?;
    if !data_url.starts_with("data:image/png;base64,") {
        return Err(format!("extract failed: {}", &data_url[..data_url.len().min(80)]));
    }

    let raw = data_url.split(',').nth(1).ok_or("no base64 data")?;
    let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, raw)
        .map_err(|e| format!("base64 decode: {}", e))?;

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("mkdir: {}", e))?;
    }
    std::fs::write(output_path, &bytes).map_err(|e| format!("write: {}", e))?;
    Ok(bytes.len())
}

/// Run a full harvest: prompt → wait → extract → save. Returns file path.
pub async fn harvest_one(config: &HarvestConfig, index: usize) -> Result<PathBuf, String> {
    let prompt = build_prompt(&config.class_name, &config.style);
    let out = config.output_dir.join(format!(
        "{}_sheet_{:03}.png",
        config.class_name, index
    ));

    type_prompt(&config.ws_url, &prompt).await?;
    click_send(&config.ws_url).await?;
    wait_for_image(&config.ws_url).await?;
    let size = extract_image(&config.ws_url, &out).await?;
    eprintln!("[harvest] saved {} ({} bytes)", out.display(), size);

    // Navigate to fresh chat for next prompt
    navigate_fresh(&config.ws_url).await?;

    Ok(out)
}

/// Run a full batch harvest for multiple classes.
pub async fn harvest_batch(
    ws_url: &str,
    output_dir: &Path,
    classes: &[(&str, usize)],
    style: &str,
) -> Result<Vec<PathBuf>, String> {
    let mut all = Vec::new();
    for &(class, count) in classes {
        for i in 0..count {
            let config = HarvestConfig {
                ws_url: ws_url.to_string(),
                output_dir: output_dir.to_path_buf(),
                class_name: class.to_string(),
                sheet_count: count,
                style: style.to_string(),
            };
            match harvest_one(&config, i).await {
                Ok(path) => all.push(path),
                Err(e) => eprintln!("[harvest] {} sheet {} failed: {}", class, i, e),
            }
        }
    }
    Ok(all)
}

/// Resolve the Chrome binary path. Honors `EXOPACK_CHROME_BIN` env var first;
/// otherwise picks a per-platform default. Returns the candidate path even if
/// it does not exist on disk — caller surfaces the spawn error.
pub fn default_chrome_bin() -> std::path::PathBuf {
    if let Ok(v) = std::env::var("EXOPACK_CHROME_BIN") {
        return std::path::PathBuf::from(v);
    }
    if cfg!(target_os = "macos") {
        std::path::PathBuf::from("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
    } else if cfg!(target_os = "windows") {
        std::path::PathBuf::from(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
    } else {
        // Linux & other unix — try `google-chrome`, fall back to `chromium`.
        for cand in ["google-chrome", "google-chrome-stable", "chromium", "chromium-browser"] {
            if let Ok(p) = which_on_path(cand) {
                return p;
            }
        }
        std::path::PathBuf::from("google-chrome")
    }
}

fn which_on_path(name: &str) -> Result<std::path::PathBuf, ()> {
    let path = std::env::var_os("PATH").ok_or(())?;
    for dir in std::env::split_paths(&path) {
        let cand = dir.join(name);
        if cand.is_file() {
            return Ok(cand);
        }
    }
    Err(())
}

/// Launch a headless Chrome with a **fresh, empty** user-data-dir for CDP automation.
///
/// Cross-platform; honors `EXOPACK_CHROME_BIN` env override.
///
/// **Security note:** does NOT copy your real Chrome profile, cookies, or login
/// state. Earlier versions did and that put authenticated session data in
/// world-readable temp — that behavior is removed. If you need an authenticated
/// Gemini session, launch your own Chrome with `--remote-debugging-port=<port>`
/// against your real profile and skip this helper. See README.
pub async fn launch_debug_chrome(port: u16) -> Result<String, String> {
    let chrome_bin = default_chrome_bin();

    // Fresh, empty profile dir — never the user's real profile.
    let profile = std::env::temp_dir().join(format!("exopack-chrome-debug-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&profile);
    std::fs::create_dir_all(&profile).map_err(|e| format!("mkdir profile: {}", e))?;

    std::process::Command::new(&chrome_bin)
        .args([
            &format!("--remote-debugging-port={}", port),
            &format!("--user-data-dir={}", profile.display()),
            "--no-first-run",
            "--no-default-browser-check",
            "--headless=new",
            "https://gemini.google.com/app",
        ])
        .spawn()
        .map_err(|e| format!("chrome launch ({}): {}", chrome_bin.display(), e))?;

    // Wait for CDP to be ready
    for _ in 0..20 {
        tokio::time::sleep(Duration::from_millis(500)).await;
        if let Ok(resp) = reqwest::get(&format!("http://localhost:{}/json/version", port)).await {
            if resp.status().is_success() {
                return Ok(format!("http://localhost:{}", port));
            }
        }
    }
    Err("chrome did not start with debug port in 10s".into())
}

// ── CDP helpers ──

async fn cdp_send(ws_url: &str, msg: &serde_json::Value) -> Result<serde_json::Value, String> {
    use futures::SinkExt;
    use futures::StreamExt;
    use tokio_tungstenite::connect_async;

    let (mut ws, _) = connect_async(ws_url)
        .await
        .map_err(|e| format!("ws connect: {}", e))?;

    ws.send(tokio_tungstenite::tungstenite::Message::Text(msg.to_string().into()))
        .await
        .map_err(|e| format!("ws send: {}", e))?;

    if let Some(Ok(msg)) = ws.next().await {
        let text = msg.to_text().map_err(|e| format!("ws text: {}", e))?;
        serde_json::from_str(text).map_err(|e| format!("ws parse: {}", e))
    } else {
        Err("ws no response".into())
    }
}

async fn cdp_eval(ws_url: &str, expression: &str) -> Result<String, String> {
    let msg = serde_json::json!({
        "id": 1,
        "method": "Runtime.evaluate",
        "params": {"expression": expression}
    });
    let resp = cdp_send(ws_url, &msg).await?;
    Ok(resp
        .pointer("/result/result/value")
        .and_then(|v| v.as_str())
        .unwrap_or("null")
        .to_string())
}

async fn cdp_eval_async(ws_url: &str, expression: &str) -> Result<String, String> {
    let msg = serde_json::json!({
        "id": 1,
        "method": "Runtime.evaluate",
        "params": {"expression": expression, "awaitPromise": true}
    });
    let resp = cdp_send(ws_url, &msg).await?;
    Ok(resp
        .pointer("/result/result/value")
        .and_then(|v| v.as_str())
        .unwrap_or("null")
        .to_string())
}
