#!/bin/bash
set -e

# eMo Unified System Installer (Professional Redesign)
# Targets: Linux, macOS, WSL

# Colors
C_BLUE='\033[0;34m'
C_GREEN='\033[0;32m'
C_RED='\033[0;31m'
C_CYAN='\033[0;36m'
C_YELLOW='\033[1;33m'
C_DIM='\033[2m'
C_NC='\033[0m'
C_BOLD='\033[1m'

# CONFIG
GITHUB_USER="Adrian-Myat-005"
REPO_URL="https://github.com/$GITHUB_USER/eMo.git"
INSTALL_DIR="$HOME/.emo"
BIN_DIR="$INSTALL_DIR/bin"

# UTILS
hide_cursor() { printf "\033[?25l"; }
show_cursor() { printf "\033[?25h"; }

# Smooth Block Progress
draw_progress() {
    local current=$1
    local total=$2
    local percent=$(( 100 * current / total ))
    local bar_len=30
    local filled=$(( bar_len * percent / 100 ))
    local empty=$(( bar_len - filled ))

    local bar=""
    for ((i=0; i<filled; i++)); do bar+="█"; done
    for ((i=0; i<empty; i++)); do bar+="░"; done

    printf "\n  ${C_DIM}Progress: [${C_NC}${C_CYAN}%s${C_NC}${C_DIM}] %3d%%${C_NC}\n" "$bar" "$percent"
}

start_task() {
    local label=$1
    printf "  ${C_BLUE}◈${C_NC} %-35s" "$label"
}

end_task_ok() {
    printf "${C_GREEN}DONE${C_NC}\n"
}

end_task_err() {
    printf "${C_RED}FAIL${C_NC}\n"
}

# --- START ---
trap show_cursor EXIT
hide_cursor
clear

echo -e "\n${C_CYAN}${C_BOLD}  eMo UNIFIED SYSTEM INSTALLATION${C_NC}"
echo -e "  ${C_DIM}════════════════════════════════════════════${C_NC}\n"

TOTAL_STEPS=6
CURRENT_STEP=0

# 1. Environment
start_task "Checking Life Support (Rust)"
if command -v cargo &> /dev/null; then
    end_task_ok
else
    end_task_err
    echo -e "\n  ${C_RED}Rust not found.${C_NC} Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi
CURRENT_STEP=1
draw_progress $CURRENT_STEP $TOTAL_STEPS

# 2. Clone
start_task "Downloading Neural Patterns"
TEMP_DIR=$(mktemp -d)
if git clone "$REPO_URL" "$TEMP_DIR" --quiet; then
    end_task_ok
else
    end_task_err
    exit 1
fi
CURRENT_STEP=2
draw_progress $CURRENT_STEP $TOTAL_STEPS

# 3. Build SadSmile
cd "$TEMP_DIR"
start_task "Compiling Core Kernel (SadSmile)"
if cargo build --release --manifest-path sadsmile/Cargo.toml --quiet; then
    end_task_ok
else
    end_task_err
    exit 1
fi
CURRENT_STEP=3
draw_progress $CURRENT_STEP $TOTAL_STEPS

# 4. Build HappyCry
start_task "Synthesizing Interface (HappyCry)"
if cargo build --release --manifest-path happycry/Cargo.toml --quiet; then
    end_task_ok
else
    end_task_err
    exit 1
fi
CURRENT_STEP=4
draw_progress $CURRENT_STEP $TOTAL_STEPS

# 5. Build eMo Compiler
start_task "Engaging Logic Gates (Compiler)"
if cargo build --release --manifest-path emo_compiler/Cargo.toml --quiet; then
    end_task_ok
else
    end_task_err
    exit 1
fi
CURRENT_STEP=5
draw_progress $CURRENT_STEP $TOTAL_STEPS

# 6. Finalizing
start_task "Optimizing Binaries & Path"
mkdir -p "$BIN_DIR"
cp sadsmile/target/release/sadsmile "$BIN_DIR/ss"
cp sadsmile/target/release/sadsmile "$BIN_DIR/nexus"
cp happycry/target/release/happy "$BIN_DIR/happy"
cp emo_compiler/target/release/emo_compiler "$BIN_DIR/emo"

SHELL_CONFIG=""
for f in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.profile"; do
    if [ -f "$f" ]; then SHELL_CONFIG="$f"; break; fi
done
if [ -n "$SHELL_CONFIG" ]; then
    if ! grep -q "$BIN_DIR" "$SHELL_CONFIG"; then
        echo >> "$SHELL_CONFIG"
        echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_CONFIG"
    fi
fi
end_task_ok
CURRENT_STEP=6
draw_progress $CURRENT_STEP $TOTAL_STEPS

# Cleanup
rm -rf "$TEMP_DIR"

echo -e "\n  ${C_GREEN}${C_BOLD}INSTALLATION COMPLETE${C_NC}"
echo -e "  ${C_DIM}────────────────────────────────────────────${C_NC}"
echo -e "  To activate, restart your terminal or run:"
echo -e "  ${C_CYAN}source $SHELL_CONFIG${C_NC}\n"
echo -e "  Type ${C_YELLOW}nexus${C_NC} to enter the eMo environment.\n"

show_cursor