#!/bin/bash
set -e # Exit on error

echo "💧 Building Sadsmile Release..."
cargo build --release

echo "📦 Installing to system path..."
# Requires sudo to write to /usr/local/bin
if [ -w /usr/local/bin ]; then
    cp target/release/sadsmile /usr/local/bin/ss
else
    echo "⚠️  Sudo required to move binary to /usr/local/bin"
    sudo cp target/release/sadsmile /usr/local/bin/ss
fi

echo "🧹 Setting permissions..."
if [ -w /usr/local/bin/ss ]; then
    chmod +x /usr/local/bin/ss
else
    sudo chmod +x /usr/local/bin/ss
fi

echo "🔧 Configuring autocompletion (User Override)..."

# 1. Clean up previous attempts (System-wide file)
if [ -f /etc/bash_completion.d/ss_sadsmile ]; then
    echo "🧹 Removing previous system-wide fix (requires sudo)..."
    sudo rm /etc/bash_completion.d/ss_sadsmile || echo "⚠️ Could not remove /etc file, you may need to remove it manually."
fi

# 2. Clean up previous attempts (.bashrc)
if [ -f "$HOME/.bashrc" ]; then
    if grep -q "Sadsmile Autocompletion Fix" "$HOME/.bashrc"; then
        echo "🧹 Cleaning up .bashrc modifications..."
        sed -i '/Sadsmile Autocompletion Fix/d' "$HOME/.bashrc"
        sed -i '/complete -o default ss/d' "$HOME/.bashrc"
    fi
fi

# 3. Install the robust fix: Local Completion Override
# Files in ~/.local/share/bash-completion/completions/ take precedence over /usr/share/...
# This "masks" the system 'ss' completion with our simple one.
COMPLETION_DIR="$HOME/.local/share/bash-completion/completions"
mkdir -p "$COMPLETION_DIR"

echo "complete -o default ss" > "$COMPLETION_DIR/ss"

echo "✅ Autocompletion override installed to $COMPLETION_DIR/ss"
echo "🎉 Sadsmile Installed! Please restart your terminal to apply changes."