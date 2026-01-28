#!/bin/bash
set -e

# eMo Unified System Installer
# Targets: Linux, macOS, WSL

# Colors
C_BLUE='\033[0;34m'
C_GREEN='\033[0;32m'
C_RED='\033[0;31m'
C_CYAN='\033[0;36m'
C_YELLOW='\033[1;33m'
C_NC='\033[0m'
C_BOLD='\033[1m'

# CONFIG
GITHUB_USER="Adrian-Myat-005"
REPO_URL="https://github.com/$GITHUB_USER/eMo.git"
INSTALL_DIR="$HOME/.emo"
BIN_DIR="$INSTALL_DIR/bin"

# UTILS: ANIMATION & PROGRESS
draw_bar() {
    # Usage: draw_bar <current_step> <total_steps> <msg>
    local current=$1
    local total=$2
    local msg=$3
    
    # Calculate percentage
    local percent=$(( 100 * current / total ))
    local bar_len=20
    local filled=$(( bar_len * percent / 100 ))
    local empty=$(( bar_len - filled ))

    # Construct bar
    local bar_str=""
    for ((i=0; i<filled; i++)); do bar_str+="█"; done
    for ((i=0; i<empty; i++)); do bar_str+="░"; done

    # Color logic for HP
    local color=$C_GREEN
    if [ $percent -lt 30 ]; then color=$C_RED;
    elif [ $percent -lt 70 ]; then color=$C_YELLOW;
    fi

    # Clear line and print
    printf "\r\033[K" # Clear line
    printf "${C_BOLD}HP: [${color}%s${C_NC}${C_BOLD}] %3d%% ${C_NC} :: %s" "$bar_str" "$percent" "$msg"
}

spinner_pid=""
start_spinner() {
    set +m
    {
        local delay=0.1
        local spinstr='|/-\'
        while :; do
            local temp=${spinstr#?}
            printf " [%c] " "$spinstr"
            local spinstr=$temp${spinstr%"$temp"}
            sleep $delay
            printf "\b\b\b\b\b"
        done
    } & 
    spinner_pid=$!
}

stop_spinner() {
    if [ -n "$spinner_pid" ]; then
        kill "$spinner_pid" >/dev/null 2>&1
        wait "$spinner_pid" >/dev/null 2>&1
        printf "\b\b\b\b\b" # Clear spinner chars
        spinner_pid=""
    fi
}

run_task() {
    local cmd=$1
    local step=$2
    local total=$3
    local label=$4

    draw_bar "$step" "$total" "$label..."
    start_spinner
    
    # Run command and capture error if any
    if eval "$cmd" > /dev/null 2>&1; then
        stop_spinner
    else
        stop_spinner
        echo -e "\n${C_RED}❌ CRITICAL DAMAGE: Failed at '$label'${C_NC}"
        echo "Command: $cmd"
        exit 1
    fi
}

# --- START ---
clear
echo -e "${C_BLUE}
███████╗███╗   ███╗ ██████╗ 
██╔════╝████╗ ████║██╔═══██╗
█████╗  ██╔████╔██║██║   ██║
██╔══╝  ██║╚██╔╝██║██║   ██║
███████╗██║ ╚═╝ ██║╚██████╔╝
╚══════╝╚═╝     ╚═╝ ╚═════╝ 
${C_NC}"
echo -e "${C_CYAN}Initializing System Link...${C_NC}\n"

# STEPS
TOTAL_STEPS=6

# 1. RUST CHECK
run_task "command -v cargo" 1 $TOTAL_STEPS "Checking Life Support (Rust)"

# 2. CLONE
TEMP_DIR=$(mktemp -d)
run_task "git clone \"$REPO_URL\" \"$TEMP_DIR\" --quiet" 2 $TOTAL_STEPS "Downloading Neural Patterns (Clone)"

# 3. BUILD
cd "$TEMP_DIR"
# SadSmile
run_task "cargo build --release --manifest-path sadsmile/Cargo.toml --quiet" 3 $TOTAL_STEPS "Compiling Core Kernel (SadSmile)"
# HappyCry
run_task "cargo build --release --manifest-path happycry/Cargo.toml --quiet" 4 $TOTAL_STEPS "Synthesizing Dopamine (HappyCry)"
# Compiler
run_task "cargo build --release --manifest-path emo_compiler/Cargo.toml --quiet" 5 $TOTAL_STEPS "Engaging Logic Gates (eMo Compiler)"

# INSTALL
mkdir -p "$BIN_DIR"
install_bin "target/release/sadsmile" "$BIN_DIR/ss"
# Create Nexus (Visual Mode)
cp "$BIN_DIR/ss" "$BIN_DIR/nexus"
echo -e "  - Installed ${C_GREEN}nexus${C_NC} (Visual Environment)"

install_bin "target/release/happy" "$BIN_DIR/happy"
cp emo_compiler/target/release/emo_compiler "$BIN_DIR/emo"

# 4. PATH
SHELL_CONFIG=""
for f in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.profile"; do
    if [ -f "$f" ]; then SHELL_CONFIG="$f"; break; fi
done

if [ -n "$SHELL_CONFIG" ]; then
    if ! grep -q "$BIN_DIR" "$SHELL_CONFIG"; then
        echo >> "$SHELL_CONFIG"
        echo "# eMo Ecosystem" >> "$SHELL_CONFIG"
        echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_CONFIG"
    fi
fi

# 5. CLEANUP
rm -rf "$TEMP_DIR"

# FINISH
draw_bar $TOTAL_STEPS $TOTAL_STEPS "SYSTEM FULLY OPERATIONAL"
echo -e "\n"
echo -e "${C_GREEN}✅ Installation Complete.${C_NC}"
echo -e "Restart your terminal or run: ${C_BLUE}source $SHELL_CONFIG${C_NC}"
echo -e "Type ${C_YELLOW}ss${C_NC} for standard shell."
echo -e "Type ${C_YELLOW}nexus${C_NC} to enter the Full eMo Environment."
