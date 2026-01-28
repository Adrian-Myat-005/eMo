# The eMo Nexus
> A Self-Healing, AI-Powered, Adaptive Web Platform.

This project demonstrates the full power of the **eMo Ecosystem**.
It combines all 4 dimensions into a single, cohesive application.

## Architecture

| Dimension | File | Role | Description |
|-----------|------|------|-------------|
| **Shadow** | `src/adapter.shw` | **The Soul** | Absorbs `petgraph` and `rapier` from GitHub and synthesizes them into native libraries (`std::graph`, `std::physics`) on startup. |
| **SadSmile** | `src/kernel.ss` | **The Body** | Allocates a zero-copy memory arena and spins up a high-performance network daemon with direct syscall access. |
| **ThinkingVirus** | `src/brain.tvrus` | **The Mind** | Spawns a custom Transformer model (AI) in RAM, trains it on local docs, and protects the system from threats in real-time. |
| **HappyCry** | `src/interface.hpy` | **The Face** | Renders a 3D GPU-accelerated globe in the browser via WebAssembly to visualize system health. |

## How to Run

```bash
# 1. Build the Unified Binary
emo build main.emo

# 2. Run (Requires Root for SadSmile Kernel access)
sudo ./nexus
```

## The "Revolutionary" Flow
1.  **Adaptation:** When you run it, `Shadow` checks if you have the libraries. If not, it clones and compiles them instantly.
2.  **Genesis:** `ThinkingVirus` doesn't call OpenAI. It *creates* a small AI model locally to serve your requests efficiently.
3.  **Performance:** `SadSmile` handles the networking so fast that `HappyCry` can stream 3D data without lag.

---
*Built with eMo v4.0*
