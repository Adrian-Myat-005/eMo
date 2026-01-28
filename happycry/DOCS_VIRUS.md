# 🧠 ThinkingVirus (`virus`)
> **Inject Intelligence into your Terminal.**

The **ThinkingVirus** is the ultimate force multiplier for HappyCry developers. It is a CLI companion that directly interfaces your codebase with the world's most powerful Large Language Models (LLMs). Whether you're using Groq, OpenAI, or a local model, the Virus reads your `.hpy` source files, understands your project's context, and "Vibe Codes" solutions directly into your workflow.

---

## ⚡ Installation

Forge the binary from source and unleash it upon your system.

### 1. Build & Install
Run the following command in the root of the repository:

```bash
cargo install --path . --bin virus
```

### 2. System Path Configuration
To command the `virus` from any directory, ensure your Cargo bin path is in your system's `PATH`.

**For Linux/macOS:**
Add this to your `.bashrc` or `.zshrc`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

**For Windows:**
Ensure `%USERPROFILE%\.cargo\bin` is in your Environment Variables.

---

## 🎛️ The Dashboard (`happy.tvrus`)

The `happy.tvrus` file is the **Brain** of your operation. It tells the ThinkingVirus where to find its intelligence and how to behave.

### Configuration Structure
This TOML-based dashboard allows full customization of the AI backend.

```toml
[package]
name = "my_happy_app"
version = "0.1.0"

[ai]
# The API Endpoint (Groq, OpenAI, or Localhost)
endpoint = "https://api.groq.com/openai/v1/chat/completions"

# The Model ID
model = "llama3-70b-8192"

# Your Secret Key
api_key = "gsk_..."

# The AI's Identity
persona = "You are a HappyCry Expert. Write secure SecOps code."
```

### Field Intelligence
- **`endpoint`**: The neural link. Connect to huge cloud brains (Groq, OpenAI) or your own local silicon (Ollama, LocalAI).
- **`model`**: The specific mind you want to summon (e.g., `gpt-4`, `llama3`, `mixtral`).
- **`api_key`**: The credential token. Keep this safe.
- **`persona`**: The personality implant. Define if you want a "SecOps Expert", a "Frontend Wizard", or a "Ruthless Refactorer".

---

## 🕹️ Commands Guide

### `virus init`
**Bootstrapping the Brain.**
Creates a default `happy.tvrus` file in your current directory. It comes pre-loaded with Groq settings for high-speed, low-latency inference.

```bash
virus init
```

### `virus vibe "[prompt]"`
**The Vibe Coding Protocol.**
This is the core loop. The Virus scans every `.hpy` file in your directory to build a full context window, then transmits your prompt to the AI.

**Workflow:**
1.  **SCAN**: Reads all `.hpy` files.
2.  **TRANSMIT**: Sends code + prompt to the endpoint.
3.  **RECEIVE**: Prints the generated solution in high-visibility green text.

**Example:**
```bash
virus vibe "Scan the project and fix the infinite loop in the port scanner task."
```
```bash
virus vibe "Generate a new page block for the admin login."
```

---

## 🌐 Provider Guide (Universal Connectivity)

The ThinkingVirus is model-agnostic. It speaks the universal language of the OpenAI Chat API standard.

### 🚀 Groq (Recommended for Speed)
*Fastest inference, generous free tier.*
```toml
[ai]
endpoint = "https://api.groq.com/openai/v1/chat/completions"
model = "llama3-70b-8192"
api_key = "gsk_..."
```

### 🧠 OpenAI (GPT-4)
*Maximum reasoning capability.*
```toml
[ai]
endpoint = "https://api.openai.com/v1/chat/completions"
model = "gpt-4-turbo"
api_key = "sk-..."
```

### 🏠 LocalAI / Ollama (Offline & Uncensored)
*Run completely offline. No data leaves your machine.*
```toml
[ai]
endpoint = "http://localhost:11434/v1/chat/completions"
model = "llama3"
api_key = "ignored" # Localhost usually doesn't require a key
```

---

## 🛡️ Best Practices

### 1. Trust, but Verify
The AI is a powerful assistant, not a compiler.
> **Rule #1:** Always review the generated code before pasting it into your `.hpy` files.

### 2. Security Hygiene
Your `happy.tvrus` file contains sensitive API keys.
> **Rule #2:** Add `happy.tvrus` to your `.gitignore` immediately.

```bash
echo "happy.tvrus" >> .gitignore
```

### 3. Context Awareness
The Virus sends **all** `.hpy` files in the current folder.
> **Rule #3:** Be mindful of sensitive data hardcoded in your source files before running `virus vibe`.

---

**Happy Hacking.** 
*The ThinkingVirus is watching.*
