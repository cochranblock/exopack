#!/usr/bin/env bash
# Copyright (c) 2026 The Cochran Block. All rights reserved.
# Build exopack for macOS ARM + Linux x86_64, output to release/
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
RELEASE_DIR="$PROJECT_DIR/release"
FEATURES="triple_sims"
LINUX_HOST="st"

mkdir -p "$RELEASE_DIR"

echo "=== macOS aarch64 ==="
cargo build --manifest-path "$PROJECT_DIR/Cargo.toml" \
    --release --features "$FEATURES" --target aarch64-apple-darwin
cp "$PROJECT_DIR/target/aarch64-apple-darwin/release/exopack" \
    "$RELEASE_DIR/exopack-macos-aarch64"
echo "  $(wc -c < "$RELEASE_DIR/exopack-macos-aarch64") bytes"

echo "=== Linux x86_64 (via $LINUX_HOST) ==="
# Vendor deps for offline build
(cd "$PROJECT_DIR" && cargo vendor --quiet)
mkdir -p "$PROJECT_DIR/.cargo"
cat > "$PROJECT_DIR/.cargo/config.toml" << 'VENDORCFG'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
VENDORCFG

# Rsync to build host, build in /tmp to avoid workspace conflicts
rsync -az --delete --exclude='target' --exclude='release' \
    "$PROJECT_DIR/" "$LINUX_HOST:~/exopack-build/"
ssh "$LINUX_HOST" "rm -rf /tmp/exopack-build && \
    cp -r ~/exopack-build /tmp/exopack-build && \
    cd /tmp/exopack-build && \
    ~/.cargo/bin/cargo build --release --features $FEATURES"
scp "$LINUX_HOST:/tmp/exopack-build/target/release/exopack" \
    "$RELEASE_DIR/exopack-linux-x86_64"
echo "  $(wc -c < "$RELEASE_DIR/exopack-linux-x86_64") bytes"

# Clean up vendor config (don't leave in repo)
rm -f "$PROJECT_DIR/.cargo/config.toml"
rm -rf "$PROJECT_DIR/vendor"
rmdir "$PROJECT_DIR/.cargo" 2>/dev/null || true

echo ""
echo "=== Release binaries ==="
ls -lh "$RELEASE_DIR"/exopack-*
echo ""
echo "Upload: gh release upload vX.Y.Z release/exopack-* --clobber"
