#!/bin/bash
set -e

# eMo "Cinema-Grade" Installer
# Designed for high-end CLI feel.

# Colors
C_BCYAN='\033[1;36m'
C_BLUE='\033[0;34m'
C_GREEN='\033[0;32m'
C_YELLOW='\033[1;33m'
C_NC='\033[0m'
C_DIM='\033[2m'

GITHUB_USER="Adrian-Myat-005"
REPO_URL="https://github.com/$GITHUB_USER/eMo.git"
INSTALL_DIR="$HOME/.emo"
BIN_DIR="$INSTALL_DIR/bin"

# UI UTILS
hide_cursor() { printf "\033[?25l"; }
show_cursor() { printf "\033[?25h"; }

# The High-FPS Spinner (Runs in background)
spinner() {
    local frames='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
    while true; do
        for (( i=0; i<${#frames}; i++ )); do
            printf "\r  ${C_BCYAN}%s${C_NC}  ${C_DIM}%s${C_NC}" "${frames:$i:1}" "$1"
            sleep 0.08
        done
    done
}

# Progress Bar
draw_bar() {
    local p=$1
    local width=30
    local filled=$(( p * width / 100 ))
    local bar=""
    for ((i=0; i<filled; i++)); do bar+="█"; done
    for ((i=0; i<width-filled; i++)); do bar+=" "; done
    printf "\n  ${C_DIM}[${C_NC}${C_BCYAN}%s${C_NC}${C_DIM}] ${C_NC}${C_BCYAN}%d%%${C_NC}\n" "$bar" "$p"
}

run_task() {
    local label=$2
    local percent=$3
    
    # Start spinner in background
    spinner "$label" &
    local SP_PID=$!
    
    # Run task
    if eval "$1" > /dev/null 2>&1; then
        kill $SP_PID >/dev/null 2>&1
        printf "\r  ${C_GREEN}✔${C_NC}  %-40s ${C_GREEN}OK${C_NC}\n" "$label"
        draw_bar "$percent"
        # Move cursor up to prepare for next update if needed, but here we stack for clarity
        printf "\033[A\033[A" # Go up 2 lines
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

# Create temp space
TEMP_DIR=$(mktemp -d)

# SEQUENCE
# We use a bit of cursor math to keep it clean
run_task "sleep 0.5" "Establishing Secure Uplink" 15
printf "\n\n" # Make space for bar

run_task "git clone \"$REPO_URL\" \"$TEMP_DIR\" --quiet" "Downloading Neural Patterns" 30
printf "\n\n"

# 3. BUILD
cd "$TEMP_DIR"

# Optimized Workspace Build (One shot for all 3 engines)
run_task "cargo build --release --quiet" "Synthesizing All Engines (Optimized)" 80

# INSTALL
mkdir -p "$BIN_DIR"
cp target/release/sadsmile "$BIN_DIR/ss"
cp target/release/sadsmile "$BIN_DIR/nexus"
cp target/release/happy "$BIN_DIR/happy"
cp target/release/emo_compiler "$BIN_DIR/emo"

# Path logic
SHELL_CONFIG="$HOME/.bashrc"
[ -f "$HOME/.zshrc" ] && SHELL_CONFIG="$HOME/.zshrc"
if ! grep -q "$BIN_DIR" "$SHELL_CONFIG"; then
    echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_CONFIG"
fi

rm -rf "$TEMP_DIR"

# FINISH
printf "\r  ${C_GREEN}✔${C_NC}  %-40s ${C_GREEN}100%%${C_NC}\n" "System Integration Complete"
draw_bar 100

echo -e "\n  ${C_BCYAN}ACCESS GRANTED.${C_NC}"
echo -e "  ${C_DIM}Restart terminal or run: source $SHELL_CONFIG${C_NC}"
echo -e "  ${C_DIM}Launch environment with:${C_NC} ${C_YELLOW}nexus${C_NC}\n"

show_cursor
