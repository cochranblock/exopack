<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Accessibility — Section 508 / WCAG — exopack

*2026-03-27.*

## Product Type

exopack is a **CLI tool and Rust library**. It has no web UI, no GUI, and no user-facing visual interface. Accessibility requirements apply to the CLI only.

## CLI Accessibility (Section 508 §1194.31)

| Requirement | Status | Evidence |
|-------------|--------|----------|
| Text output readable by screen readers | Pass | All output is plain text to stdout/stderr. No ANSI color codes. No cursor manipulation. |
| `--help` flag | Pass | `exopack --help` prints usage, commands, examples. Exits 0. |
| `--version` flag | Pass | `exopack --version` prints "exopack 0.1.0". Exits 0. |
| Exit codes | Pass | 0 = success, 1 = error. Standard POSIX convention. |
| Error messages | Pass | All errors go to stderr with context: what failed, where, why. |
| No color-only information | Pass | No colored output. All status communicated via text (OK/FAIL, percentages). |
| Keyboard-only operation | Pass | CLI is keyboard-only by nature. No mouse interaction. |

## Library Output Accessibility

The `t63::print_summary()` method (visual regression report) outputs plain text:
```
SIM 4 VISUAL REGRESSION: 3 pages
  [OK] index — 0.12% diff (within 1.0% threshold)
  [FAIL] resume — 4.73% diff (exceeds 1.0% threshold)
  [OK] mural — 0.00% diff (within 1.0% threshold)
SIM 4: 2/3 pages OK
```

- Status conveyed by text `[OK]`/`[FAIL]`, not color
- Numeric diff percentages provided for screen reader users
- Summary line at end with pass/total count

## Not Applicable

- WCAG color contrast: No visual UI
- ARIA labels: No web interface
- Touch targets: No touch interface
- Font sizes: No rendered text (screenshots are test artifacts, not user-facing)
