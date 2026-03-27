// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! f70 = screenshot — out_dir, theme, capture_project, compare_screenshots, generate_diff_image.
//! Visual regression: pixel-level diff (f71) and red-highlight diff image (f72).

use std::path::PathBuf;

/// f70_out_dir. Returns cache dir for screenshots: ~/.cache/screenshots/{os}/{project}
pub fn out_dir(project: &str) -> PathBuf {
    let base = dirs::cache_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("screenshots")
        .join(std::env::consts::OS);
    base.join(project)
}

/// Theme for cochranblock: block diagram styling.
#[derive(Clone)]
pub struct Theme {
    _placeholder: (),
}

/// f70_theme_cochranblock. Cochranblock block-diagram theme.
pub fn theme_cochranblock() -> Theme {
    Theme { _placeholder: () }
}

/// f70_capture_project. Fetches each page, renders via headless browser (devtools) or placeholder.
/// Returns true if all captures succeed.
pub async fn capture_project(
    base: &str,
    project: &str,
    pages: &[(&str, &str)],
    _theme: &Theme,
) -> bool {
    let dir = out_dir(project);
    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("screenshot: mkdir {}: {}", dir.display(), e);
        return false;
    }

    #[cfg(feature = "devtools")]
    {
        match crate::devtools::capture_screenshots(base, pages, &dir).await {
            Ok(ok) => return ok,
            Err(e) => {
                eprintln!("screenshot: devtools fallback to placeholder: {}", e);
            }
        }
    }

    capture_placeholder(base, project, pages, &dir).await
}

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

/// Result of comparing two screenshots pixel-by-pixel.
pub struct CompareResult {
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
pub fn compare_screenshots(
    actual: &std::path::Path,
    baseline: &std::path::Path,
    tolerance: u8,
    threshold: f64,
) -> Result<CompareResult, String> {
    let img_a = image::open(actual).map_err(|e| format!("open {}: {}", actual.display(), e))?;
    let img_b = image::open(baseline).map_err(|e| format!("open {}: {}", baseline.display(), e))?;

    let rgba_a = img_a.to_rgba8();
    let rgba_b = img_b.to_rgba8();

    // Resize actual to baseline dimensions if they differ
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
        let differs = pa.0[..3].iter().zip(pb.0[..3].iter()).any(|(&a, &b)| {
            (a as i16 - b as i16).abs() > tol
        });
        if differs {
            diff_pixels += 1;
        }
    }

    let diff_pct = if total_pixels == 0 {
        0.0
    } else {
        (diff_pixels as f64 / total_pixels as f64) * 100.0
    };

    Ok(CompareResult {
        matches: diff_pct <= threshold,
        diff_pct,
        total_pixels,
        diff_pixels,
    })
}

/// f72 = generate_diff_image. Creates a visual diff PNG highlighting changed pixels in red.
pub fn generate_diff_image(
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
        let differs = pa.0[..3].iter().zip(pb.0[..3].iter()).any(|(&a, &b)| {
            (a as i16 - b as i16).abs() > tol
        });
        *pixel = if differs {
            image::Rgba([255, 0, 0, 200])
        } else {
            image::Rgba([pb.0[0], pb.0[1], pb.0[2], 80])
        };
    }

    diff_img.save(out).map_err(|e| format!("save {}: {}", out.display(), e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_dir_contains_os_and_project() {
        let p = out_dir("myproject");
        let s = p.to_string_lossy();
        assert!(s.contains(std::env::consts::OS), "path should contain OS: {}", s);
        assert!(s.ends_with("myproject"), "path should end with project name: {}", s);
    }

    #[test]
    fn compare_identical_images() {
        let dir = std::env::temp_dir().join("exopack_test_compare");
        let _ = std::fs::create_dir_all(&dir);
        let path_a = dir.join("a.png");
        let path_b = dir.join("b.png");

        let img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([100, 150, 200, 255]));
        img.save(&path_a).unwrap();
        img.save(&path_b).unwrap();

        let result = compare_screenshots(&path_a, &path_b, 10, 1.0).unwrap();
        assert!(result.matches);
        assert_eq!(result.diff_pct, 0.0);
        assert_eq!(result.diff_pixels, 0);
        assert_eq!(result.total_pixels, 100);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn compare_different_images() {
        let dir = std::env::temp_dir().join("exopack_test_diff");
        let _ = std::fs::create_dir_all(&dir);
        let path_a = dir.join("a.png");
        let path_b = dir.join("b.png");

        let white = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([255, 255, 255, 255]));
        let black = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([0, 0, 0, 255]));
        white.save(&path_a).unwrap();
        black.save(&path_b).unwrap();

        let result = compare_screenshots(&path_a, &path_b, 10, 1.0).unwrap();
        assert!(!result.matches);
        assert_eq!(result.diff_pct, 100.0);
        assert_eq!(result.diff_pixels, 100);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn diff_image_generates_file() {
        let dir = std::env::temp_dir().join("exopack_test_diffimg");
        let _ = std::fs::create_dir_all(&dir);
        let path_a = dir.join("a.png");
        let path_b = dir.join("b.png");
        let path_d = dir.join("diff.png");

        let white = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([255, 255, 255, 255]));
        let black = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([0, 0, 0, 255]));
        white.save(&path_a).unwrap();
        black.save(&path_b).unwrap();

        generate_diff_image(&path_a, &path_b, &path_d, 10).unwrap();
        assert!(path_d.exists());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
