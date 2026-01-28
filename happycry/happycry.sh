#!/bin/bash
set -e

# 1. Define Variables
INSTALL_DIR="$HOME/.happycry"
# Replace the URL below with your actual GitHub release link after uploading
DOWNLOAD_URL="https://github.com/Adrian-Myat-005/happycry/releases/download/v1.0.0/happycry-linux-x86_64.tar.gz"

echo "🌟 Installing HappyCry..."

# 2. Prepare Directory
mkdir -p "$INSTALL_DIR"

# 3. Download
echo "⬇️  Downloading distribution..."
TEMP_FILE="$INSTALL_DIR/happycry.tar.gz"

if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$DOWNLOAD_URL" -o "$TEMP_FILE"
elif command -v wget >/dev/null 2>&1; then
    wget -qO "$TEMP_FILE" "$DOWNLOAD_URL"
else
    echo "❌ Error: Neither curl nor wget found."
    exit 1
fi

# 4. Install (Extract)
echo "📦 Extracting..."
tar -xzf "$TEMP_FILE" -C "$INSTALL_DIR"

# 5. Path Setup
BIN_DIR="$INSTALL_DIR/bin"
SHELL_CONFIG=""
case "$SHELL" in
    */zsh) SHELL_CONFIG="$HOME/.zshrc" ;; 
    */bash) SHELL_CONFIG="$HOME/.bashrc" ;; 
    *) SHELL_CONFIG="$HOME/.profile" ;; 
esac

if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo "" >> "$SHELL_CONFIG"
    echo "# HappyCry Path" >> "$SHELL_CONFIG"
    echo "export PATH=\"$PATH:$BIN_DIR\"" >> "$SHELL_CONFIG"
    echo "✅ Added $BIN_DIR to $SHELL_CONFIG"
fi

# 6. Cleanup
rm "$TEMP_FILE"

# 7. Final Output
echo "💉 HappyCry Installed. Restart your terminal."
