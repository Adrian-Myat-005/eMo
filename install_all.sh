#!/bin/bash
set -e

# eMo "Cinema-Grade" Installer v4.2
# High-end CLI with real-time download and build percentages.

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

draw_bar() {
    local p=$1
    local label=$2
    local width=40
    local filled=$(( p * width / 100 ))
    local bar=""
    for ((i=0; i<filled; i++)); do bar+="█"; done
    for ((i=0; i<width-filled; i++)); do bar+=" "; done
    
    printf "\r\033[K" # Clear line
    printf "  ${C_BLUE}◈${C_NC} %-30s ${C_BCYAN}%3d%%${C_NC} [${C_BCYAN}%s${C_NC}]" "$label" "$p" "$bar"
}

# --- START ---
trap show_cursor EXIT
hide_cursor
clear

echo -e "\n  ${C_BCYAN}eMo ECOSYSTEM :: DEPLOYMENT SEQUENCE${C_NC}"
echo -e "  ${C_DIM}──────────────────────────────────────────${C_NC}\n"

# 1. Environment Check
printf "  ${C_BLUE}✔${C_NC} Checking Life Support (Rust)... "
if command -v cargo &> /dev/null; then echo -e "${C_GREEN}OK${C_NC}"; else echo -e "${C_RED}MISSING${C_NC}"; exit 1; fi

# 2. DOWNLOAD WITH PERCENTAGE
echo -e "  ${C_BLUE}◈${C_NC} ${C_DIM}Downloading Neural Patterns (Source Data)...${C_NC}"
TEMP_DIR=$(mktemp -d)
show_cursor
if ! curl -L "$REPO_ZIP" -# -o "$TEMP_DIR/source.tar.gz"; then
    echo -e "\n  ${C_RED}✘ Download Failed${C_NC}"; exit 1
fi
hide_cursor
printf "\033[A\033[K" # Go up and clear curl line
printf "\r  ${C_GREEN}✔${C_NC} Downloading Neural Patterns                ${C_GREEN}100%%${C_NC}\n"

# 3. EXTRACTION
printf "  ${C_BLUE}◈${C_NC} Unpacking Neural Segments... "
tar -xzf "$TEMP_DIR/source.tar.gz" -C "$TEMP_DIR" --strip-components=1
echo -e "${C_GREEN}OK${C_NC}"

# 4. COMPILATION WITH PERCENTAGE
cd "$TEMP_DIR"
echo -e "  ${C_BLUE}◈${C_NC} ${C_DIM}Synthesizing All Engines (This takes time)...${C_NC}"

# We use cargo's progress info to calculate percentage
# Logic: We capture the "Compiling X/Y" lines
# Pattern: [X/Y]
cargo build --release 2>&1 | while read -r line; do
    if [[ "$line" =~ \[([0-9]+)/([0-9]+)\] ]]; then
        current=${BASH_REMATCH[1]}
        total=${BASH_REMATCH[2]}
        pct=$(( 100 * current / total ))
        draw_bar "$pct" "Compiling Engines"
    fi
done

printf "\r\033[K"
printf "  ${C_GREEN}✔${C_NC} Synthesizing All Engines                 ${C_GREEN}100%%${C_NC}\n"

# INSTALL
mkdir -p "$BIN_DIR"
cp target/release/sadsmile "$BIN_DIR/ss"
cp target/release/sadsmile "$BIN_DIR/nexus"
cp target/release/happy "$BIN_DIR/happy"
cp target/release/emo_compiler "$BIN_DIR/emo"

# Remove legacy installations
if [ -d "$HOME/.happycry" ]; then
    echo -e "  ${C_YELLOW}Removing legacy ~/.happycry installation...${C_NC}"
    rm -rf "$HOME/.happycry"
fi
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
exec "$BIN_DIR/nexus"
