<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# exopack Compression Map (P13)

All public symbols use compressed identifiers per Kova P13 protocol.
Doc comments on each symbol map to human-readable names.

## Functions (f+num)

| Token | Human Name | Module | Signature |
|-------|-----------|--------|-----------|
| f60 | triple_sims_run | triple_sims | `async fn f60<F,Fut>(run_once: F) -> bool` |
| f61 | run_cargo_test_n | triple_sims | `fn f61(project_dir, n) -> (bool, String)` |
| f61_with_args | run_cargo_test_n + args | triple_sims | `fn f61_with_args(project_dir, n, args) -> (bool, String)` |
| f62 | live_demo | triple_sims | `fn f62_live_demo(project_dir, bin_name, cargo_args) -> io::Result<ExitStatus>` |
| f63 | discover_test_bin | triple_sims | `fn f63_discover_test_bin(project_dir) -> Option<String>` |
| f70 | out_dir | screenshot | `fn f70(project) -> PathBuf` |
| f71 | compare_screenshots | screenshot | `fn f71(actual, baseline, tolerance, threshold) -> Result<t61, String>` |
| f72 | generate_diff_image | screenshot | `fn f72(actual, baseline, out, tolerance) -> Result<(), String>` |
| f73 | visual_regression | screenshot | `async fn f73(base_url, project, pages, tolerance, threshold) -> t63` |
| f74 | check_console_errors | devtools | `async fn f74(base, paths) -> Result<Vec<String>, String>` |
| f75 | capture_screenshots | devtools | `async fn f75(base, pages, out_dir) -> Result<bool, String>` |
| f76 | update_baselines | screenshot | `fn f76(project, pages) -> Result<u32, String>` |
| f77 | baseline_dir | screenshot | `fn f77(project) -> PathBuf` |
| f78 | theme_cochranblock | screenshot | `fn f78() -> t60` |
| f79 | capture_project | screenshot | `async fn f79(base, project, pages, theme) -> bool` |
| f80 | bind_random | interface | `async fn f80() -> Result<(TcpListener, String), String>` |
| f81 | http_client | interface | `fn f81() -> Result<reqwest::Client, String>` |
| f82 | start_mock_server | mock | `async fn f82() -> (MockServer, String)` |
| f83 | mount_get | mock | `async fn f83(server, path, body)` |
| f84 | mount_get_json | mock | `async fn f84(server, path, json)` |
| f85 | mount_post | mock | `async fn f85(server, path, body)` |
| f86 | mount_post_json | mock | `async fn f86(server, path, json)` |
| f87 | mount_status | mock | `async fn f87(server, path, status, body)` |
| f88 | capture_screenshot | video | `fn f88(out_dir, name) -> Result<PathBuf, String>` |
| f89 | create_recorder | video | `fn f89() -> Box<dyn t64>` |
| f90 | demo_dir | demo | `fn f90() -> PathBuf` |
| f95 | run_baked_demo | baked_demo | `async fn f95(kova_bin, home, port) -> Result<(), String>` |

## Types (t+num)

| Token | Human Name | Module | Kind |
|-------|-----------|--------|------|
| t60 | Theme | screenshot | struct |
| t61 | CompareResult | screenshot | struct |
| t62 | PageResult | screenshot | struct |
| t63 | VisualReport | screenshot | struct |
| t64 | VideoRecorder | video | trait |
| t65 | NoopRecorder | video | struct |
| t66 | DemoAction | demo | enum |
| t67 | DemoRecord | demo | struct |

## Fields (s+num)

### t61 (CompareResult)
| Token | Field | Type |
|-------|-------|------|
| s60 | matches | bool |
| s61 | diff_pct | f64 |
| s62 | total_pixels | u32 |
| s63 | diff_pixels | u32 |

### t62 (PageResult)
| Token | Field | Type |
|-------|-------|------|
| s64 | name | String |
| s65 | passed | bool |
| s66 | diff_pct | f64 |
| s67 | actual | PathBuf |
| s68 | baseline | PathBuf |
| s69 | diff_image | Option<PathBuf> |
| s70 | status | String |

### t63 (VisualReport)
| Token | Field | Type |
|-------|-------|------|
| s71 | pages | Vec<t62> |
| s72 | all_passed | bool |
| s73 | baselines_created | u32 |
| s74 | baselines_compared | u32 |

### t67 (DemoRecord)
| Token | Field | Type |
|-------|-------|------|
| s75 | name | String |
| s76 | source | String |
| s77 | actions | Vec<t66> |
| s78 | started_at | Option<String> |

## CLI Commands (c+num)

| Token | Command | Binary |
|-------|---------|--------|
| c60 | live-demo | exopack |

## Standards Check (f100+, t70+)

| Token | Human Name | Module | Signature |
|-------|-----------|--------|-----------|
| f100 | print_table | standards_check | `fn print_table(&self)` (on t72) |
| f101 | check_project | standards_check | `fn f101(project_dir) -> t71` |
| f102 | check_clippy | standards_check | `fn f102(dir) -> t70` |
| f103 | check_fmt | standards_check | `fn f103(dir) -> t70` |
| f104 | check_audit | standards_check | `fn f104(dir) -> t70` |
| f105 | check_deny | standards_check | `fn f105(dir) -> t70` |
| f106 | check_msrv | standards_check | `fn f106(cargo_content) -> t70` |
| f107 | check_unsafe | standards_check | `fn f107(dir) -> t70` |
| f108 | check_docs | standards_check | `fn f108(dir) -> t70` |
| f109 | check_changelog | standards_check | `fn f109(dir) -> t70` |
| f110 | check_license_file | standards_check | `fn f110(dir) -> t70` |
| f111 | check_test_binary | standards_check | `fn f111(dir, cargo) -> t70` |
| f112 | check_allow_unused | standards_check | `fn f112(dir) -> t70` |
| f113 | check_error_handling | standards_check | `fn f113(dir) -> t70` |
| f114 | check_secrets | standards_check | `fn f114(dir) -> t70` |
| f115 | check_cargo_meta | standards_check | `fn f115(cargo_content) -> t70` |
| f116 | check_portfolio | standards_check | `fn f116(projects) -> t72` |
| t70 | CheckResult | standards_check | struct { s80: name, s81: passed, s82: detail } |
| t71 | ProjectReport | standards_check | struct { s83: name, s84: path, s85: checks } |
| t72 | PortfolioReport | standards_check | struct { s86: reports } |

## Ranges

| Range | Module |
|-------|--------|
| f60–f63 | triple_sims |
| f70–f79 | screenshot |
| f74–f75 | devtools |
| f80–f81 | interface |
| f82–f87 | mock |
| f88–f89 | video |
| f90 | demo |
| f95 | baked_demo |
| f100–f116 | standards_check |
| t60–t63 | screenshot |
| t64–t65 | video |
| t66–t67 | demo |
| t70–t72 | standards_check |
| s60–s63 | t61 fields |
| s64–s70 | t62 fields |
| s71–s74 | t63 fields |
| s75–s78 | t67 fields |
| s80–s86 | t70–t72 fields |
