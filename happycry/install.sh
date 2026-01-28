#!/bin/bash

# --- HAPPYCRY INJECTION SCRIPT ---
# "Injecting Intelligence..."

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

HAPPY_HOME="$HOME/.happycry"
BIN_DIR="$HAPPY_HOME/bin"
LIB_DIR="$HAPPY_HOME/lib"
CONFIG_DIR="$HAPPY_HOME/config"

echo -e "${CYAN}💉 Initializing HappyCry Injection Sequence...${NC}"

# 1. Dependency Check
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust toolchain missing.${NC} System incompatible."
    echo "Please install Rust first: https://rustup.rs"
    exit 1
fi

# 2. Directory Setup
echo -e "${CYAN}📂 constructing neural pathways at ${HAPPY_HOME}...${NC}"
mkdir -p "$BIN_DIR"
mkdir -p "$LIB_DIR"
mkdir -p "$CONFIG_DIR"

# 3. Build Process
echo -e "${CYAN}🔨 Compiling binaries (This may take a moment)...${NC}"

# Simple spinner function
spinner() {
    local pid=$1
    local delay=0.1
    local spinstr='|/-\'
    while [ "$(ps a | awk '{print $1}' | grep $pid)" ]; do
        local temp=${spinstr#?}
        printf " [%c]  " "$spinstr"
        local spinstr=$temp${spinstr%"$temp"}
        sleep $delay
        printf "\b\b\b\b\b\b"
    done
    printf "    \b\b\b\b"
}

# Run build in background to show spinner
cargo build --release --bins > /dev/null 2>&1 &
BUILD_PID=$!
spinner $BUILD_PID

wait $BUILD_PID
BUILD_EXIT_CODE=$?

if [ $BUILD_EXIT_CODE -ne 0 ]; then
    echo -e "\n${RED}❌ Build Failed.${NC} Check cargo output for details."
    # Run again to show error
    cargo build --release --bins
    exit 1
fi

echo -e "${GREEN}✅ Compilation Successful.${NC}"

# 4. Installation
echo -e "${CYAN}📦 Moving artifacts to ${BIN_DIR}...${NC}"
if [ -f "target/release/happy" ]; then
    cp "target/release/happy" "$BIN_DIR/"
else
    echo -e "${RED}❌ Error: 'happy' binary not found.${NC}"
    exit 1
fi

if [ -f "target/release/virus" ]; then
    cp "target/release/virus" "$BIN_DIR/"
else
    echo -e "${YELLOW}⚠️  Warning: 'virus' binary not found.${NC}"
fi

# 5. Path Configuration
SHELL_CONFIG=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
else
    # Fallback/Guess
    if [ -f "$HOME/.zshrc" ]; then
        SHELL_CONFIG="$HOME/.zshrc"
    elif [ -f "$HOME/.bashrc" ]; then
        SHELL_CONFIG="$HOME/.bashrc"
    fi
fi

if [ -n "$SHELL_CONFIG" ]; then
    echo -e "${CYAN}🔗 Linking to system shell ($SHELL_CONFIG)...${NC}"
    EXPORT_CMD="export PATH=\"$BIN_DIR:\$PATH\""
    
    if grep -q "$BIN_DIR" "$SHELL_CONFIG"; then
        echo -e "${YELLOW}⚡ Path already configured.${NC}"
    else
        echo >> "$SHELL_CONFIG"
        echo "# HappyCry Toolchain" >> "$SHELL_CONFIG"
        echo "$EXPORT_CMD" >> "$SHELL_CONFIG"
        echo -e "${GREEN}✅ Path injected.${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  Could not detect shell config file. Manually add this to your path:${NC}"
    echo "   export PATH=\"$BIN_DIR:\$PATH\""
fi

echo -e "\n${GREEN}=========================================${NC}"
echo -e "${GREEN} 💉 INJECTION COMPLETE. HAPPYCRY IS ACTIVE.${NC}"
echo -e "${GREEN}=========================================${NC}"
echo -e "Restart your terminal or run: ${YELLOW}source $SHELL_CONFIG${NC}"
