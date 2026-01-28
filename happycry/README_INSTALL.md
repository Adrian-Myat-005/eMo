# HappyCry Installation Guide

> "Inject Intelligence into your System."

This repository contains automated injection scripts to build and install the HappyCry ecosystem (`happy` compiler + `virus` AI tool) directly from source.

## Prerequisites

*   **Rust Toolchain**: You must have Rust installed (`cargo`).
    *   **Install**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## 🐧 Linux / 🍎 macOS

1.  **Grant Permission**:
    ```bash
    chmod +x install.sh
    ```

2.  **Run Injection**:
    ```bash
    ./install.sh
    ```

3.  **Finalize**:
    Restart your terminal or run `source ~/.bashrc` (or `~/.zshrc`).

## 🪟 Windows

1.  **Run with PowerShell**:
    Right-click `install.ps1` and select **"Run with PowerShell"**.
    
    *Or run from terminal:*
    ```powershell
    .\install.ps1
    ```

    *Note: You may need to enable script execution if restricted:*
    ```powershell
    Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
    ```

2.  **Finalize**:
    Restart your terminal (Command Prompt or PowerShell) to refresh environment variables.

## Verify Installation

Check if the tools are active:

```bash
happy --version
# Should output happy version

virus --help
# Should show ThinkingVirus help
```

**Happy Hacking.**
