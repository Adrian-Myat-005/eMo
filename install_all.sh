#!/bin/bash
set -e

# eMo Unified System Installer
# Targets: Linux, macOS, WSL

# Colors
C_BLUE='\033[0;34m'
C_GREEN='\033[0;32m'
C_RED='\033[0;31m'
C_NC='\033[0m'

# REPLACE THIS WITH YOUR USERNAME
GITHUB_USER="Adrian-Myat-005"
REPO_URL="https://github.com/$GITHUB_USER/eMo.git"

INSTALL_DIR="$HOME/.emo"
BIN_DIR="$INSTALL_DIR/bin"

echo -e "${C_BLUE}Initializing eMo System Installation...${C_NC}"

# 1. Check for Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${C_RED}Rust not found.${C_NC} Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# 2. Create Temp Workspace
TEMP_DIR=$(mktemp -d)
echo -e "Cloning repository to $TEMP_DIR..."
git clone "$REPO_URL" "$TEMP_DIR" --quiet

# 3. Build & Install
mkdir -p "$BIN_DIR"

echo "Building eMo Engines (this may take a few minutes)..."
cd "$TEMP_DIR"

# Build All in Workspace
# This builds all members defined in root Cargo.toml
cargo build --release --quiet

echo "Installing binaries..."

# Helper function to install
install_bin() {
    SRC=$1
    DEST=$2
    if [ -f "$SRC" ]; then
        cp "$SRC" "$DEST"
        echo -e "  - Installed ${C_GREEN}$(basename $DEST)${C_NC}"
    else
        echo -e "${C_RED}Error: Binary not found at $SRC${C_NC}"
        # Fallback check for nested targets (legacy support)
        if [ -f "sadsmile/$SRC" ]; then cp "sadsmile/$SRC" "$DEST"; return; fi
        if [ -f "happycry/$SRC" ]; then cp "happycry/$SRC" "$DEST"; return; fi
        if [ -f "emo_compiler/$SRC" ]; then cp "emo_compiler/$SRC" "$DEST"; return; fi
        exit 1
    fi
}

install_bin "target/release/sadsmile" "$BIN_DIR/ss"
install_bin "target/release/happy" "$BIN_DIR/happy"
install_bin "target/release/emo_compiler" "$BIN_DIR/emo"

# 4. Finalize Path
SHELL_CONFIG=""
for f in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.profile"; do
    if [ -f "$f" ]; then
        SHELL_CONFIG="$f"
        break
    fi
done

if [ -n "$SHELL_CONFIG" ]; then
    if ! grep -q "$BIN_DIR" "$SHELL_CONFIG"; then
        echo -e "\n# eMo Ecosystem\nexport PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_CONFIG"
    fi
    echo -e "${C_GREEN}Installation Successful!${C_NC}"
    echo -e "Restart your terminal or run: ${C_BLUE}source $SHELL_CONFIG${C_NC}"
else
    echo -e "${C_GREEN}Installation Successful!${C_NC}"
    echo -e "Add this to your PATH: ${C_BLUE}export PATH=\"$BIN_DIR:\$PATH\"${C_NC}"
fi

# 5. Cleanup
rm -rf "$TEMP_DIR"