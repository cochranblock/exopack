<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Privacy Impact Assessment — exopack

*2026-03-27.*

## Data Collection

exopack collects **no user data**. It is a build-time testing tool that runs locally.

| Data type | Collected? | Details |
|-----------|-----------|---------|
| PII (names, emails, SSN) | No | Not applicable |
| User analytics | No | No telemetry, no phone-home, no usage tracking |
| Credentials | No | No authentication system |
| IP addresses | No | HTTP requests go to localhost test servers only |
| Browser history | No | Chromium (devtools feature) runs headless against test URLs only |

## Data Storage

| Artifact | Location | Contents | PII risk |
|----------|----------|----------|----------|
| Screenshots | `~/.cache/screenshots/{os}/{project}/` | PNG images of test pages | None — test pages, not user content |
| Baselines | `~/.cache/screenshots/{os}/{project}/baselines/` | Golden screenshot PNGs | None |
| Diff images | `~/.cache/screenshots/{os}/{project}/diffs/` | Red-highlight diff PNGs | None |
| Demo recordings | `~/.kova/demos/` | JSON action scripts | None — synthetic test actions |

All data is local. Nothing leaves the machine. No cloud storage. No external APIs contacted except:
- The test server under evaluation (localhost)
- Chromium download (devtools feature, one-time, from official distribution)

## GDPR / CCPA Applicability

**Not applicable.** exopack does not process, store, or transmit personal data of any kind. It is a developer tool that operates exclusively on test artifacts in local filesystem caches.

## Data Retention

All artifacts are local files. Users control retention. `rm -rf ~/.cache/screenshots` removes all screenshot data. No remote backups.

## Third-Party Data Sharing

None. exopack makes no network requests to third-party services. The only network activity is HTTP requests to localhost test servers (controlled by the user's test binary).
