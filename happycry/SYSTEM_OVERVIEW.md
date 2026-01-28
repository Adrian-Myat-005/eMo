# 💧 HappyCry & 🧠 ThinkingVirus: System Overview

This document provides a comprehensive context for the `HappyCry` project, designed to be ingested by an AI assistant to facilitate deep discussion, debugging, and feature development.

---

## 1. Project Identity
- **Name**: HappyCry (`.hpy`)
- **Type**: Domain-Specific Language (DSL) & AI-Assisted Development Environment.
- **Core Philosophy**: "Write like a human, compile like a machine."
- **Host Language**: Rust (1.70+).
- **Primary Target**: High-performance binaries for SecOps, Web, and Distributed Systems.

---

## 2. Core Components

### A. The HappyCry Compiler (`happy`)
A multi-stage pipeline that transforms `.hpy` scripts into native machine code via Rust.
1.  **Lexer (`src/lexer.rs`)**: Tokenizes human-readable syntax.
2.  **Parser (`src/parser.rs`)**: Constructs an Abstract Syntax Tree (AST).
3.  **AST (`src/ast.rs`)**: The structural representation of the program.
4.  **Codegen (`src/codegen.rs`)**: Translates AST nodes into optimized Rust code.
5.  **Builder (`src/builder.rs`)**: Manages the `rustc` invocation and final binary linkage.

### B. The ThinkingVirus (`virus`)
A CLI-based "Vibe Coding" companion (`src/bin/virus/`).
- **Function**: Scans the codebase for context and interacts with LLMs (Groq, OpenAI, Ollama).
- **Configuration**: Managed via `happy.tvrus` (TOML).
- **Workflow**: `virus vibe "instruction"` reads all local `.hpy` files and generates code suggestions based on the project's specific DSL syntax.

---

## 3. Language Features (The `.hpy` Ecosystem)
- **HappyWeb**: Full-stack web development using Actix-Web integration (e.g., `page "/" do ... end`).
- **SecOps**: Built-in async networking (Tokio) and TCP connectivity checks.
- **Swarm Mode**: Distributed P2P networking powered by `libp2p` (Gossipsub/mDNS).
- **HappyBase**: Embedded persistence via SQLite.
- **Universal Bridge**: FFI support for linking C libraries.
- **Escape Hatch**: Raw Rust injection using `#happy-raw ... #happy-end`.

---

## 4. Technical Stack
- **Runtime**: Tokio (Async/Await).
- **Web**: Actix-Web.
- **Networking**: Libp2p (P2P), Reqwest (HTTP).
- **Data**: Serde (JSON/TOML), SQLite.
- **UI/Terminal**: Colored (CLI output).

---

## 5. Directory Structure
```text
.
├── src/
│   ├── main.rs          # 'happy' compiler entry point
│   ├── lexer.rs         # Tokenization logic
│   ├── parser.rs        # AST construction
│   ├── ast.rs           # Syntax tree definitions
│   ├── codegen.rs       # Rust code generation
│   └── bin/virus/       # 'virus' AI CLI source
├── DOCS_HAPPYCRY.md     # Language reference
├── DOCS_VIRUS.md        # AI companion documentation
├── Cargo.toml           # Project dependencies
└── web.hpy              # Example HappyCry script
```

---

## 6. Development Workflow
- **Compile a script**: `happy build script.hpy`
- **Configure AI**: `virus init` (Creates `happy.tvrus`)
- **AI-Assisted Coding**: `virus vibe "Add a login route to the web server"`

---

## 7. Syntax & Implementation Notes
- **File Header**: Scripts often start with `#happy` to denote the DSL version or type.
- **Server Syntax**: Implementation in `web.hpy` uses `server new port [var]` and `route "[path]"` followed by `serve [var]`, which may vary from the planned reference in `DOCS_HAPPYCRY.md`.
- **Variables**: Strings and numeric types are supported via `set [name] to [value]`.
- **Output**: `say` is the primary print command.

---

## 8. Strategic Goals for AI Discussion
1.  **Feature Expansion**: Implementing new DSL keywords (e.g., `match`, `try-catch`).
2.  **Optimizer Improvements**: Enhancing the `codegen` phase for better Rust output.
3.  **Virus Integration**: Improving how the AI reads and writes `.hpy` files directly.
4.  **SecOps Modules**: Adding native support for more network protocols.
