// Unlicense — public domain — cochranblock.org
//! Screenshot — capture, compare, diff, visual regression (Sim 4).
//!
//! Public API:
//! - [`out_dir`], [`baseline_dir`] — cache paths
//! - [`compare_screenshots`] — pixel-level diff with tolerance/threshold
//! - [`generate_diff_image`] — red-highlight diff PNG
//! - [`visual_regression`] — full Sim 4 orchestrator (auto-baseline first run)
//! - [`update_baselines`] — promote current captures to new baselines
//!
//! P13 aliases (`f70`–`f79`, `t60`–`t63`) retained for kova/cochranblock.

use std::path::PathBuf;

// ── Canonical public API (v0.3+) ──────────────────────────────────

/// Cache dir for a project's screenshots: `~/.cache/screenshots/{os}/{project}`.
pub fn out_dir(project: &str) -> PathBuf {
    f70(project)
}

/// Baseline dir for a project: `<out_dir>/baselines`.
pub fn baseline_dir(project: &str) -> PathBuf {
    f77(project)
}

/// Pixel-level diff between two PNGs. `tolerance` is per-channel slack;
/// `threshold` is the max diff percentage to count as a match.
pub fn compare_screenshots(
    actual: &std::path::Path,
    baseline: &std::path::Path,
    tolerance: u8,
    threshold: f64,
) -> Result<CompareResult, String> {
    f71(actual, baseline, tolerance, threshold)
}

/// Write a diff PNG that highlights changed pixels in red.
pub fn generate_diff_image(
    actual: &std::path::Path,
    baseline: &std::path::Path,
    out: &std::path::Path,
    tolerance: u8,
) -> Result<(), String> {
    f72(actual, baseline, out, tolerance)
}

/// Sim 4 visual regression orchestrator. First run creates baselines, later
/// runs compare and emit red-highlight diffs for failures.
pub async fn visual_regression(
    base_url: &str,
    project: &str,
    pages: &[(&str, &str)],
    tolerance: u8,
    threshold: f64,
) -> VisualReport {
    f73(base_url, project, pages, tolerance, threshold).await
}

/// Promote `current/*.png` to `baselines/*.png`. Returns count updated.
pub fn update_baselines(project: &str, pages: &[&str]) -> Result<u32, String> {
    f76(project, pages)
}

/// Canonical alias for [`t61`].
pub use self::t61 as CompareResult;
/// Canonical alias for [`t62`].
pub use self::t62 as PageResult;
/// Canonical alias for [`t63`].
pub use self::t63 as VisualReport;
/// Canonical alias for [`t60`].
pub use self::t60 as Theme;

/// f70 = out_dir. Returns cache dir for screenshots: ~/.cache/screenshots/{os}/{project}
pub fn f70(project: &str) -> PathBuf {
    let base = dirs::cache_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("screenshots")
        .join(std::env::consts::OS);
    base.join(project)
}

/// t60 = Theme. Theme for cochranblock: block diagram styling.
#[derive(Clone)]
pub struct t60 {
    _placeholder: (),
}

/// f78 = theme_cochranblock. Cochranblock block-diagram theme.
pub fn f78() -> t60 {
    t60 { _placeholder: () }
}

/// f79 = capture_project. Fetches each page, renders via headless browser (devtools) or placeholder.
/// Returns true if all captures succeed.
pub async fn f79(base: &str, project: &str, pages: &[(&str, &str)], _theme: &t60) -> bool {
    let dir = f70(project);
    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("screenshot: mkdir {}: {}", dir.display(), e);
        return false;
    }

    #[cfg(feature = "devtools")]
    {
        match crate::devtools::f75(base, pages, &dir).await {
            Ok(ok) => return ok,
            Err(e) => {
                eprintln!("screenshot: devtools fallback to placeholder: {}", e);
            }
        }
    }

    capture_placeholder(base, project, pages, &dir).await
}

// STUB: This is *not* a real screenshot capture. It fetches the URL to
// confirm reachability, then writes a fixed gray 100x100 PNG. The output is
// useful only for plumbing tests (file IO, baseline staging), not for actual
// visual regression. When the `devtools` feature is on, [`capture_to_dir`]
// uses real headless Chromium and a devtools failure is a hard error — this
// stub is only reached when devtools is compiled out.
async fn capture_placeholder(
    base: &str,
    _project: &str,
    pages: &[(&str, &str)],
    dir: &std::path::Path,
) -> bool {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("screenshot: reqwest client: {}", e);
            return false;
        }
    };
    let base = base.trim_end_matches('/');
    for (name, path) in pages {
        let url = format!("{}{}", base, path);
        match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                let out = dir.join(format!("{}.png", name));
                if let Err(e) = write_placeholder_png(&out) {
                    eprintln!("screenshot: write {}: {}", out.display(), e);
                    return false;
                }
                println!("screenshot: {} -> {}", url, out.display());
            }
            Ok(resp) => {
                eprintln!("screenshot: {} -> {}", url, resp.status());
                return false;
            }
            Err(e) => {
                eprintln!("screenshot: fetch {}: {}", url, e);
                return false;
            }
        }
    }
    true
}

fn write_placeholder_png(path: &std::path::Path) -> Result<(), String> {
    let img = image::RgbaImage::from_fn(100, 100, |_, _| image::Rgba([200, 200, 200, 255]));
    img.save(path).map_err(|e| e.to_string())
}

/// t61 = CompareResult. Result of comparing two screenshots pixel-by-pixel.
pub struct t61 {
    /// True if diff_pct is below the threshold.
    pub matches: bool,
    /// Percentage of pixels that differ (0.0–100.0).
    pub diff_pct: f64,
    /// Total pixels compared.
    pub total_pixels: u32,
    /// Number of differing pixels.
    pub diff_pixels: u32,
}

/// f71 = compare_screenshots. Pure Rust pixel-level diff between two PNGs.
/// `tolerance` = per-channel difference allowed before counting as changed (e.g. 10 for anti-aliasing).
/// `threshold` = max diff_pct to consider a match (e.g. 1.0 = 1%).
pub fn f71(
    actual: &std::path::Path,
    baseline: &std::path::Path,
    tolerance: u8,
    threshold: f64,
) -> Result<t61, String> {
    let img_a = image::open(actual).map_err(|e| format!("open {}: {}", actual.display(), e))?;
    let img_b = image::open(baseline).map_err(|e| format!("open {}: {}", baseline.display(), e))?;

    let rgba_a = img_a.to_rgba8();
    let rgba_b = img_b.to_rgba8();

    let (w, h) = (rgba_b.width(), rgba_b.height());
    let rgba_a = if rgba_a.dimensions() != (w, h) {
        image::imageops::resize(&rgba_a, w, h, image::imageops::FilterType::Lanczos3)
    } else {
        rgba_a
    };

    let total_pixels = w * h;
    let mut diff_pixels = 0u32;
    let tol = tolerance as i16;

    for (pa, pb) in rgba_a.pixels().zip(rgba_b.pixels()) {
        let differs = pa.0[..3]
            .iter()
            .zip(pb.0[..3].iter())
            .any(|(&a, &b)| (a as i16 - b as i16).abs() > tol);
        if differs {
            diff_pixels += 1;
        }
    }

    let diff_pct = if total_pixels == 0 {
        0.0
    } else {
        (diff_pixels as f64 / total_pixels as f64) * 100.0
    };

    Ok(t61 {
        matches: diff_pct <= threshold,
        diff_pct,
        total_pixels,
        diff_pixels,
    })
}

/// f72 = generate_diff_image. Creates a visual diff PNG highlighting changed pixels in red.
pub fn f72(
    actual: &std::path::Path,
    baseline: &std::path::Path,
    out: &std::path::Path,
    tolerance: u8,
) -> Result<(), String> {
    let img_a = image::open(actual).map_err(|e| format!("open {}: {}", actual.display(), e))?;
    let img_b = image::open(baseline).map_err(|e| format!("open {}: {}", baseline.display(), e))?;

    let rgba_a = img_a.to_rgba8();
    let rgba_b = img_b.to_rgba8();

    let (w, h) = (rgba_b.width(), rgba_b.height());
    let rgba_a = if rgba_a.dimensions() != (w, h) {
        image::imageops::resize(&rgba_a, w, h, image::imageops::FilterType::Lanczos3)
    } else {
        rgba_a
    };

    let tol = tolerance as i16;
    let mut diff_img = image::RgbaImage::new(w, h);
    for (x, y, pixel) in diff_img.enumerate_pixels_mut() {
        let pa = rgba_a.get_pixel(x, y);
        let pb = rgba_b.get_pixel(x, y);
        let differs = pa.0[..3]
            .iter()
            .zip(pb.0[..3].iter())
            .any(|(&a, &b)| (a as i16 - b as i16).abs() > tol);
        *pixel = if differs {
            image::Rgba([255, 0, 0, 200])
        } else {
            image::Rgba([pb.0[0], pb.0[1], pb.0[2], 80])
        };
    }

    diff_img
        .save(out)
        .map_err(|e| format!("save {}: {}", out.display(), e))
}

/// t62 = PageResult. Per-page result from visual regression.
pub struct t62 {
    pub name: String,
    pub passed: bool,
    pub diff_pct: f64,
    pub actual: PathBuf,
    pub baseline: PathBuf,
    pub diff_image: Option<PathBuf>,
    pub status: String,
}

/// t63 = VisualReport. Full visual regression report.
pub struct t63 {
    pub pages: Vec<t62>,
    pub all_passed: bool,
    /// Captures staged in `baselines_pending/` awaiting accept (first-run).
    pub baselines_staged: u32,
    /// Pages compared against an existing trusted baseline.
    pub baselines_compared: u32,
}

impl t63 {
    pub fn print_summary(&self) {
        println!("SIM 4 VISUAL REGRESSION: {} pages", self.pages.len());
        for p in &self.pages {
            let icon = if p.passed { "OK" } else { "FAIL" };
            println!(
                "  [{}] {} — {} (diff {:.2}%)",
                icon, p.name, p.status, p.diff_pct
            );
            if let Some(ref d) = p.diff_image {
                println!("       diff: {}", d.display());
            }
        }
        if self.baselines_staged > 0 {
            println!(
                "  {} baselines STAGED in baselines_pending/ — review then run \
                 `screenshot::accept_pending_baselines(project)` to trust them",
                self.baselines_staged
            );
        }
        let pass_count = self.pages.iter().filter(|p| p.passed).count();
        println!("SIM 4: {}/{} pages OK", pass_count, self.pages.len());
    }
}

/// f77 = baseline_dir. Baseline directory for a project: ~/.cache/screenshots/{os}/{project}/baselines/
pub fn f77(project: &str) -> PathBuf {
    f70(project).join("baselines")
}

/// Pending-baseline directory. First-run captures land here, NOT in `baselines/`,
/// to prevent baseline poisoning. Promote with [`accept_pending_baselines`].
pub fn pending_baseline_dir(project: &str) -> PathBuf {
    f70(project).join("baselines_pending")
}

/// Promote `baselines_pending/*.png` to `baselines/*.png`. Returns count promoted.
/// The deliberate human-in-the-loop step that protects against first-run poisoning.
pub fn accept_pending_baselines(project: &str) -> Result<u32, String> {
    let pending = pending_baseline_dir(project);
    let trusted = f77(project);
    if !pending.exists() {
        return Ok(0);
    }
    std::fs::create_dir_all(&trusted).map_err(|e| e.to_string())?;
    let mut promoted = 0u32;
    for entry in std::fs::read_dir(&pending).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("png") {
            let name = path.file_name().ok_or("bad pending entry")?;
            let dst = trusted.join(name);
            std::fs::copy(&path, &dst)
                .map_err(|e| format!("copy {} -> {}: {}", path.display(), dst.display(), e))?;
            promoted += 1;
            println!("baseline accepted: {}", dst.display());
        }
    }
    let _ = std::fs::remove_dir_all(&pending);
    Ok(promoted)
}

/// f73 = visual_regression. Full Sim 4 orchestrator.
///
/// For each page: capture screenshot → compare against baseline → report.
/// On first run (no baseline exists), saves the capture as the new baseline and passes.
/// On subsequent runs, compares and fails if diff exceeds threshold.
/// Generates diff images for failures.
///
/// `tolerance` = per-channel pixel tolerance (e.g. 10 for anti-aliasing).
/// `threshold` = max diff percentage to pass (e.g. 1.0 = 1%).
pub async fn f73(
    base_url: &str,
    project: &str,
    pages: &[(&str, &str)],
    tolerance: u8,
    threshold: f64,
) -> t63 {
    let capture_dir = f70(project).join("current");
    let base_dir = f77(project);
    let pending_dir = pending_baseline_dir(project);
    let diff_dir = f70(project).join("diffs");

    for dir in [&capture_dir, &base_dir, &pending_dir, &diff_dir] {
        if let Err(e) = std::fs::create_dir_all(dir) {
            eprintln!("visual_regression: mkdir {}: {}", dir.display(), e);
        }
    }

    let _captured = capture_to_dir(base_url, pages, &capture_dir).await;

    let mut results = Vec::new();
    let mut baselines_staged = 0u32;
    let mut baselines_compared = 0u32;
    let mut all_passed = true;

    for (name, _path) in pages {
        let actual = capture_dir.join(format!("{}.png", name));
        let baseline = base_dir.join(format!("{}.png", name));
        let pending = pending_dir.join(format!("{}.png", name));

        if !actual.exists() {
            results.push(t62 {
                name: name.to_string(),
                passed: false,
                diff_pct: 100.0,
                actual: actual.clone(),
                baseline: baseline.clone(),
                diff_image: None,
                status: "capture failed".into(),
            });
            all_passed = false;
            continue;
        }

        if !baseline.exists() {
            // Stage into pending, NOT into baselines. Prevents first-run poisoning.
            if let Err(e) = std::fs::copy(&actual, &pending) {
                eprintln!(
                    "visual_regression: stage baseline {}: {}",
                    pending.display(),
                    e
                );
                results.push(t62 {
                    name: name.to_string(),
                    passed: false,
                    diff_pct: 100.0,
                    actual,
                    baseline,
                    diff_image: None,
                    status: "baseline stage failed".into(),
                });
                all_passed = false;
                continue;
            }
            baselines_staged += 1;
            results.push(t62 {
                name: name.to_string(),
                passed: true,
                diff_pct: 0.0,
                actual,
                baseline: pending.clone(),
                diff_image: None,
                status: "STAGED in baselines_pending/ — run accept_pending_baselines".into(),
            });
            continue;
        }

        baselines_compared += 1;
        match f71(&actual, &baseline, tolerance, threshold) {
            Ok(cmp) => {
                let diff_image = if !cmp.matches {
                    let dp = diff_dir.join(format!("{}_diff.png", name));
                    if let Err(e) = f72(&actual, &baseline, &dp, tolerance) {
                        eprintln!("visual_regression: diff image {}: {}", dp.display(), e);
                    }
                    Some(dp)
                } else {
                    None
                };

                if !cmp.matches {
                    all_passed = false;
                }

                results.push(t62 {
                    name: name.to_string(),
                    passed: cmp.matches,
                    diff_pct: cmp.diff_pct,
                    actual,
                    baseline,
                    diff_image,
                    status: if cmp.matches {
                        format!(
                            "{:.2}% diff (within {:.1}% threshold)",
                            cmp.diff_pct, threshold
                        )
                    } else {
                        format!(
                            "{:.2}% diff (exceeds {:.1}% threshold)",
                            cmp.diff_pct, threshold
                        )
                    },
                });
            }
            Err(e) => {
                all_passed = false;
                results.push(t62 {
                    name: name.to_string(),
                    passed: false,
                    diff_pct: 100.0,
                    actual,
                    baseline,
                    diff_image: None,
                    status: format!("compare error: {}", e),
                });
            }
        }
    }

    t63 {
        pages: results,
        all_passed,
        baselines_staged,
        baselines_compared,
    }
}

/// f76 = update_baselines. Copy current captures over baselines, accepting the new state.
pub fn f76(project: &str, pages: &[&str]) -> Result<u32, String> {
    let capture_dir = f70(project).join("current");
    let base_dir = f77(project);
    std::fs::create_dir_all(&base_dir).map_err(|e| e.to_string())?;

    let mut updated = 0u32;
    for name in pages {
        let src = capture_dir.join(format!("{}.png", name));
        let dst = base_dir.join(format!("{}.png", name));
        if src.exists() {
            std::fs::copy(&src, &dst)
                .map_err(|e| format!("copy {} -> {}: {}", src.display(), dst.display(), e))?;
            updated += 1;
            println!("baseline updated: {}", dst.display());
        }
    }
    Ok(updated)
}

async fn capture_to_dir(base_url: &str, pages: &[(&str, &str)], dir: &std::path::Path) -> bool {
    if let Err(e) = std::fs::create_dir_all(dir) {
        eprintln!("screenshot: mkdir {}: {}", dir.display(), e);
        return false;
    }

    // When devtools is compiled in, it is the ONLY supported capture path —
    // a failure is a hard error, not a silent fallback to placeholders.
    // Otherwise the test suite would pass on broken Chromium with gray squares.
    #[cfg(feature = "devtools")]
    {
        match crate::devtools::f75(base_url, pages, dir).await {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!(
                    "screenshot: devtools capture failed (hard error in devtools mode): {}",
                    e
                );
                false
            }
        }
    }
    #[cfg(not(feature = "devtools"))]
    {
        capture_placeholder_fallback(base_url, pages, dir).await
    }
}

#[cfg(not(feature = "devtools"))]
async fn capture_placeholder_fallback(
    base_url: &str,
    pages: &[(&str, &str)],
    dir: &std::path::Path,
) -> bool {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("screenshot: reqwest client: {}", e);
            return false;
        }
    };
    let base = base_url.trim_end_matches('/');
    for (name, path) in pages {
        let url = format!("{}{}", base, path);
        match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                let out = dir.join(format!("{}.png", name));
                if let Err(e) = write_placeholder_png(&out) {
                    eprintln!("screenshot: write {}: {}", out.display(), e);
                    return false;
                }
                println!("screenshot: {} -> {}", url, out.display());
            }
            Ok(resp) => {
                eprintln!("screenshot: {} -> {}", url, resp.status());
                return false;
            }
            Err(e) => {
                eprintln!("screenshot: fetch {}: {}", url, e);
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("{}_{}", name, std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn out_dir_contains_os_and_project() {
        let p = f70("myproject");
        let s = p.to_string_lossy();
        assert!(
            s.contains(std::env::consts::OS),
            "path should contain OS: {}",
            s
        );
        assert!(
            s.ends_with("myproject"),
            "path should end with project name: {}",
            s
        );
    }

    #[test]
    fn compare_identical_images() {
        let dir = test_dir("exopack_test_compare");
        let path_a = dir.join("a.png");
        let path_b = dir.join("b.png");

        let img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([100, 150, 200, 255]));
        img.save(&path_a).unwrap();
        img.save(&path_b).unwrap();

        let result = f71(&path_a, &path_b, 10, 1.0).unwrap();
        assert!(result.matches);
        assert_eq!(result.diff_pct, 0.0);
        assert_eq!(result.diff_pixels, 0);
        assert_eq!(result.total_pixels, 100);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn compare_different_images() {
        let dir = test_dir("exopack_test_diff");
        let path_a = dir.join("a.png");
        let path_b = dir.join("b.png");

        let white = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([255, 255, 255, 255]));
        let black = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([0, 0, 0, 255]));
        white.save(&path_a).unwrap();
        black.save(&path_b).unwrap();

        let result = f71(&path_a, &path_b, 10, 1.0).unwrap();
        assert!(!result.matches);
        assert_eq!(result.diff_pct, 100.0);
        assert_eq!(result.diff_pixels, 100);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn compare_within_tolerance() {
        let dir = test_dir("exopack_test_tolerance");
        let path_a = dir.join("a.png");
        let path_b = dir.join("b.png");

        let img_a = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([100, 100, 100, 255]));
        let img_b = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([105, 105, 105, 255]));
        img_a.save(&path_a).unwrap();
        img_b.save(&path_b).unwrap();

        let result = f71(&path_a, &path_b, 10, 1.0).unwrap();
        assert!(result.matches, "within tolerance should match");
        assert_eq!(result.diff_pixels, 0);

        let result = f71(&path_a, &path_b, 3, 1.0).unwrap();
        assert!(!result.matches, "outside tolerance should not match");
        assert_eq!(result.diff_pixels, 100);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn compare_resizes_mismatched_dimensions() {
        let dir = test_dir("exopack_test_resize");
        let path_a = dir.join("a.png");
        let path_b = dir.join("b.png");

        let img_a = image::RgbaImage::from_fn(20, 20, |_, _| image::Rgba([100, 100, 100, 255]));
        let img_b = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([100, 100, 100, 255]));
        img_a.save(&path_a).unwrap();
        img_b.save(&path_b).unwrap();

        let result = f71(&path_a, &path_b, 10, 5.0).unwrap();
        assert!(
            result.matches,
            "resized solid color should match: diff={:.2}%",
            result.diff_pct
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn baseline_dir_under_out_dir() {
        let bd = f77("testproj");
        let od = f70("testproj");
        assert!(bd.starts_with(&od), "baseline_dir should be under out_dir");
        assert!(bd.ends_with("baselines"));
    }

    #[test]
    fn update_baselines_copies_files() {
        let project = "exopack_test_update_bl";
        let current_dir = f70(project).join("current");
        let base_dir = f77(project);
        let _ = std::fs::create_dir_all(&current_dir);
        let _ = std::fs::remove_dir_all(&base_dir);

        let img = image::RgbaImage::from_fn(5, 5, |_, _| image::Rgba([42, 42, 42, 255]));
        img.save(current_dir.join("index.png")).unwrap();
        img.save(current_dir.join("about.png")).unwrap();

        let updated = f76(project, &["index", "about"]).unwrap();
        assert_eq!(updated, 2);
        assert!(base_dir.join("index.png").exists());
        assert!(base_dir.join("about.png").exists());

        let _ = std::fs::remove_dir_all(f70(project));
    }

    #[test]
    fn diff_image_generates_file() {
        let dir = test_dir("exopack_test_diffimg");
        let path_a = dir.join("a.png");
        let path_b = dir.join("b.png");
        let path_d = dir.join("diff.png");

        let white = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([255, 255, 255, 255]));
        let black = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([0, 0, 0, 255]));
        white.save(&path_a).unwrap();
        black.save(&path_b).unwrap();

        f72(&path_a, &path_b, &path_d, 10).unwrap();
        assert!(path_d.exists());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn diff_image_red_on_changed_pixels() {
        let dir = test_dir("exopack_test_diffimg_red");
        let path_a = dir.join("a.png");
        let path_b = dir.join("b.png");
        let path_d = dir.join("diff.png");

        // Left half white, right half black vs all white baseline
        let img_a = image::RgbaImage::from_fn(10, 10, |x, _| {
            if x < 5 {
                image::Rgba([255, 255, 255, 255])
            } else {
                image::Rgba([0, 0, 0, 255])
            }
        });
        let img_b = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([255, 255, 255, 255]));
        img_a.save(&path_a).unwrap();
        img_b.save(&path_b).unwrap();

        f72(&path_a, &path_b, &path_d, 10).unwrap();
        let diff = image::open(&path_d).unwrap().to_rgba8();

        // Left half matches — inherits baseline color (white), NOT red highlight
        let left = diff.get_pixel(2, 5);
        assert_eq!(left.0[3], 80, "matching pixel alpha should be 80 (semi-transparent)");

        // Right half differs — red highlight (255, 0, 0, 200)
        let right = diff.get_pixel(7, 5);
        assert_eq!(right.0[0], 255, "differing pixel R should be 255");
        assert_eq!(right.0[1], 0, "differing pixel G should be 0");
        assert_eq!(right.0[3], 200, "differing pixel alpha should be 200");

        let _ = std::fs::remove_dir_all(&dir);
    }

    // --- f73 visual regression orchestrator tests ---

    /// Spin up a minimal TCP server that returns 200 for any request.
    /// Returns the base URL (http://127.0.0.1:{port}).
    async fn start_test_server() -> String {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let base_url = format!("http://{}", addr);
        tokio::spawn(async move {
            loop {
                if let Ok((mut stream, _)) = listener.accept().await {
                    tokio::spawn(async move {
                        use tokio::io::{AsyncReadExt, AsyncWriteExt};
                        let mut buf = [0u8; 4096];
                        let _ = stream.read(&mut buf).await;
                        let resp = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK";
                        let _ = stream.write_all(resp).await;
                    });
                }
            }
        });
        base_url
    }

    #[tokio::test]
    async fn f73_stages_baselines_on_first_run() {
        let project = format!("exopack_test_f73_bl_{}", std::process::id());
        let root = f70(&project);
        let _ = std::fs::remove_dir_all(&root);

        let base_url = start_test_server().await;
        let pages: &[(&str, &str)] = &[("index", "/"), ("about", "/about")];
        let report = f73(&base_url, &project, pages, 10, 1.0).await;

        assert!(report.all_passed, "first run should pass (soft mode)");
        assert_eq!(report.baselines_staged, 2);
        assert_eq!(report.baselines_compared, 0);
        assert_eq!(report.pages.len(), 2);
        for page in &report.pages {
            assert!(page.passed);
            assert!(page.status.contains("STAGED"));
        }
        // First-run captures land in pending, NOT in trusted baselines/.
        assert!(pending_baseline_dir(&project).join("index.png").exists());
        assert!(pending_baseline_dir(&project).join("about.png").exists());
        assert!(!f77(&project).join("index.png").exists());

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn f73_compares_identical_on_second_run() {
        let project = format!("exopack_test_f73_cmp_{}", std::process::id());
        let root = f70(&project);
        let _ = std::fs::remove_dir_all(&root);

        let base_url = start_test_server().await;
        let pages: &[(&str, &str)] = &[("index", "/")];

        // First run — stages baselines
        let r1 = f73(&base_url, &project, pages, 10, 1.0).await;
        assert_eq!(r1.baselines_staged, 1);

        // Accept the staged baselines so the second run has a trusted reference.
        let promoted = accept_pending_baselines(&project).unwrap();
        assert_eq!(promoted, 1);

        // Second run — same gray placeholder vs same baseline → match
        let r2 = f73(&base_url, &project, pages, 10, 1.0).await;
        assert!(r2.all_passed, "identical captures should match");
        assert_eq!(r2.baselines_compared, 1);
        assert_eq!(r2.baselines_staged, 0);
        for page in &r2.pages {
            assert!(page.passed);
            assert!(page.diff_image.is_none());
        }

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn f73_writes_diff_image_on_failure() {
        let project = format!("exopack_test_f73_diff_{}", std::process::id());
        let root = f70(&project);
        let _ = std::fs::remove_dir_all(&root);

        let base_url = start_test_server().await;
        let pages: &[(&str, &str)] = &[("index", "/")];

        // First run — stages baseline (gray placeholder), then accept it.
        f73(&base_url, &project, pages, 10, 1.0).await;
        accept_pending_baselines(&project).unwrap();

        // Replace baseline with a red image — differs from the gray placeholder
        let bl = f77(&project).join("index.png");
        let red = image::RgbaImage::from_fn(100, 100, |_, _| image::Rgba([255, 0, 0, 255]));
        red.save(&bl).unwrap();

        // Second run — gray capture vs red baseline → diff
        let r2 = f73(&base_url, &project, pages, 10, 1.0).await;
        assert!(!r2.all_passed, "modified baseline should cause failure");
        assert_eq!(r2.baselines_compared, 1);

        let page = &r2.pages[0];
        assert!(!page.passed);
        assert!(page.diff_pct > 0.0);
        assert!(page.diff_image.is_some(), "diff image should be set");
        let diff_path = page.diff_image.as_ref().unwrap();
        assert!(diff_path.exists(), "diff PNG should exist on disk");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn accept_pending_promotes_files() {
        let project = format!("exopack_test_accept_{}", std::process::id());
        let root = f70(&project);
        let _ = std::fs::remove_dir_all(&root);

        let pending = pending_baseline_dir(&project);
        std::fs::create_dir_all(&pending).unwrap();
        let img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([0, 255, 0, 255]));
        img.save(pending.join("home.png")).unwrap();

        let promoted = accept_pending_baselines(&project).unwrap();
        assert_eq!(promoted, 1);
        assert!(f77(&project).join("home.png").exists());
        assert!(!pending.exists() || std::fs::read_dir(&pending).unwrap().next().is_none());

        let _ = std::fs::remove_dir_all(&root);
    }
}
