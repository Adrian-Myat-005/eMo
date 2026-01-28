#!/bin/bash
set -e

echo "🚀 Starting Release Build..."

# 1. Clean & Build
echo "🧹 Cleaning previous builds..."
cargo clean

echo "📦 Building 'happy' and 'virus' (release mode)..."
cargo build --release

# 2. Stage
echo "📂 Staging artifacts..."
rm -rf dist
mkdir -p dist/bin

# 3. Copy
cp target/release/happy dist/bin/
cp target/release/virus dist/bin/

# 4. Package
echo "🗜️  Compressing package..."
# We use -C dist . so the archive contains 'bin/happy' etc., not 'dist/bin/happy'
tar -czvf happycry-linux-x86_64.tar.gz -C dist .

# 5. Notify
echo "✅ Package Ready. Upload 'happycry-linux-x86_64.tar.gz' to GitHub Releases."