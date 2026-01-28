#!/bin/bash
set -e

# eMo "Cinema-Grade" Installer v4.1
# Optimized for speed and real-time data feedback.

# Colors
C_BCYAN='\033[1;36m'
C_BLUE='\033[0;34m'
C_GREEN='\033[0;32m'
C_YELLOW='\033[1;33m'
C_NC='\033[0m'
C_DIM='\033[2m'
C_RED='\033[0;31m'

GITHUB_USER="Adrian-Myat-005"
REPO_ZIP="https://github.com/$GITHUB_USER/eMo/archive/refs/heads/main.tar.gz"
INSTALL_DIR="$HOME/.emo"
BIN_DIR="$INSTALL_DIR/bin"

# UI UTILS
hide_cursor() { printf "\033[?25l"; }
show_cursor() { printf "\033[?25h"; }

# The High-FPS Spinner
spinner() {
    local frames='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
    while true; do
        for (( i=0; i<${#frames}; i++ )); do
            printf "\r  ${C_BCYAN}%s${C_NC}  ${C_DIM}%s${C_NC}" "${frames:$i:1}" "$1"
            sleep 0.08
        done
    done
}

run_task() {
    local cmd=$1
    local label=$2
    
    spinner "$label" &
    local SP_PID=$!
    
    if eval "$cmd" > /dev/null 2>&1; then
        kill $SP_PID >/dev/null 2>&1
        printf "\r  ${C_GREEN}✔${C_NC}  %-40s ${C_GREEN}OK${C_NC}\n" "$label"
    else
        kill $SP_PID >/dev/null 2>&1
        printf "\r  ${C_RED}✘${C_NC}  %-40s ${C_RED}FAILED${C_NC}\n" "$label"
        show_cursor
        exit 1
    fi
}

# --- BOOT ---
trap show_cursor EXIT
hide_cursor
clear

echo -e "\n  ${C_BCYAN}eMo ECOSYSTEM :: DEPLOYMENT SEQUENCE${C_NC}"
echo -e "  ${C_DIM}──────────────────────────────────────────${C_NC}\n"

# 1. Environment Check
run_task "command -v cargo" "Checking Life Support (Rust)"
run_task "command -v tar" "Verifying Extraction Tools"

# 2. REAL DATA DOWNLOAD
echo -e "  ${C_BLUE}◈${C_NC}  ${C_DIM}Downloading Neural Patterns (Source Data)...${C_NC}"
TEMP_DIR=$(mktemp -d)
# Using curl with a progress bar for "Real Data" feel
show_cursor
if ! curl -L "$REPO_ZIP" -# -o "$TEMP_DIR/source.tar.gz"; then
    echo -e "\n  ${C_RED}✘ Download Interrupted${C_NC}"
    exit 1
fi
hide_cursor

# 3. EXTRACTION
run_task "tar -xzf $TEMP_DIR/source.tar.gz -C $TEMP_DIR --strip-components=1" "Unpacking Neural Segments"

# 4. OPTIMIZED BUILD
cd "$TEMP_DIR"
echo -e "  ${C_BLUE}◈${C_NC}  ${C_DIM}Synthesizing All Engines (This takes time)...${C_NC}"
# We show a custom spinner for the long build
spinner "Compiling Unified Workspace" &
BUILD_SP_PID=$!

if cargo build --release --quiet; then
    kill $BUILD_SP_PID >/dev/null 2>&1
    printf "\r  ${C_GREEN}✔${C_NC}  %-40s ${C_GREEN}OK${C_NC}\n" "Synthesizing All Engines"
else
    kill $BUILD_SP_PID >/dev/null 2>&1
    printf "\r  ${C_RED}✘${C_NC}  %-40s ${C_RED}FAILED${C_NC}\n" "Synthesizing All Engines"
    show_cursor
    exit 1
fi

# 5. INSTALLATION
mkdir -p "$BIN_DIR"
cp target/release/sadsmile "$BIN_DIR/ss"
cp target/release/sadsmile "$BIN_DIR/nexus"
cp target/release/happy "$BIN_DIR/happy"
cp target/release/emo_compiler "$BIN_DIR/emo"

# Path logic
SHELL_CONFIG="$HOME/.bashrc"
[ -f "$HOME/.zshrc" ] && SHELL_CONFIG="$HOME/.zshrc"
if ! grep -q "$BIN_DIR" "$SHELL_CONFIG"; then
    echo -e "\n# eMo Ecosystem\nexport PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_CONFIG"
fi

rm -rf "$TEMP_DIR"

# FINISH
echo -e "\n  ${C_GREEN}✔${C_NC}  ${C_BOLD}System Integration Complete${C_NC}"
echo -e "  ${C_DIM}──────────────────────────────────────────${C_NC}"
echo -e "  To activate, run: ${C_BCYAN}source $SHELL_CONFIG${C_NC}"
echo -e "  Type ${C_YELLOW}nexus${C_NC} to begin.\n"

show_cursor
# Self-execute nexus if terminal allows
exec "$BIN_DIR/nexus"