import os
import random

def generate_lessons():
    base_dir = 'lessons'
    os.makedirs(base_dir, exist_ok=True)
    
    # Curriculum Stages with Folder Names
    stages = [
        (0, 2500, ".ss", "sys", "1_SadSmile_Core", "Robust Kernel Control"),
        (2500, 5000, ".hpy", "joy", "2_HappyCry_Interface", "Fluid UI Experience"),
        (5000, 7500, ".tvrus", "mind", "3_ThinkingVirus_Genesis", "AI Model Spawning"),
        (7500, 10000, ".shw", "void", "4_Shadow_Adaptation", "Library Synthesis")
    ]

    topics = {
        ".ss": [
            "Zero-Copy Buffer", "Kernel Mutex", "SIMD Vectorization", "Memory Arena", 
            "Inline Assembly", "Driver Interrupt", "Atomic Reference", "Pointer Arithmetic", 
            "Stack Allocation", "Syscall Hook"
        ],
        ".hpy": [
            "Virtual DOM", "GPU Shader", "Event Loop", "Reactive State", 
            "Flexbox Layout", "WebAssembly Target", "Touch Gesture", "Audio Stream", 
            "Canvas Rendering", "HMR (Hot Reload)"
        ],
        ".tvrus": [
            "Spawn Transformer", "Backpropagation", "Weight Quantization", "LoRA Fine-Tuning",
            "Attention Head", "Token Embeddings", "Model Pruning", "RAG Pipeline", 
            "Hyperparameter Opt", "Neural Architecture Search"
        ],
        ".shw": [
            "Absorb GitHub Repo", "Synthesize C++ Lib", "Transpile Python", "Extract API Contract",
            "Dependency Resolution", "AST Transformation", "Binary Patching", "Protocol Reverse Eng",
            "Auto-Documentation", "Cross-Platform Build"
        ]
    }

    print("Generating 10,000 Lessons into Structured Folders...")

    for start, end, ext, lib, folder_name, desc in stages:
        # Create the sub-folder
        folder_path = os.path.join(base_dir, folder_name)
        os.makedirs(folder_path, exist_ok=True)
        
        for i in range(start, end):
            lesson_num = i + 1
            topic = random.choice(topics[ext])
            
            # Filename inside the folder
            filename = os.path.join(folder_path, f"lesson_{lesson_num:05d}{ext}.md")
            
            code = get_code(ext, lib, topic)
            
            if ext == ".hpy":
                # HappyCry is top-level script, no main function needed
                content = f"""# Lesson {lesson_num:05d}: {topic}
## Module: {desc} ({folder_name})

### 1. Code
```emo
# {topic.replace(" ", "_").lower()}{ext}
# Using library: {lib}

{code}
```

### 2. Mechanics
*   **Target:** {folder_name.replace("_", " ")} Runtime.
*   **Operation:** {get_explanation(ext)}
*   **Efficiency:** O(1) execution cost.

### 3. Task
1.  Implement `{topic}` in `{ext}`.
2.  Compile with `happy build`.
"""
            else:
                # eMo/SadSmile/Shadow use Rust-like main function
                content = f"""# Lesson {lesson_num:05d}: {topic}
## Module: {desc} ({folder_name})

### 1. Code
```emo
// {topic.replace(" ", "_").lower()}{ext}
import {lib}

fn main() {{
{code}
}}
```

### 2. Mechanics
*   **Target:** {folder_name.replace("_", " ")} Runtime.
*   **Operation:** {get_explanation(ext)}
*   **Efficiency:** O(1) execution cost.

### 3. Task
1.  Implement `{topic}` in `{ext}`.
2.  Compile with `emo build`.
"""
            with open(filename, 'w') as f:
                f.write(content)

def get_code(ext, lib, topic):
    if ext == ".ss":
        return f"""    // Direct Memory Access for {topic}
    unsafe {{
        let ptr = {lib}.mem.alloc(1024)
        {lib}.asm("mov rax, 1") // Raw Speed
        {lib}.mem.free(ptr)
    }}"""
    elif ext == ".hpy":
        return f"""    # Rendering {topic}
    set view to {lib}.ui.create("{topic}")
    call view.style("opacity", 0.9)
    call view.style("blur", 10)
    
    async task render_loop do
        loop 60 times do
             call ctx.gpu_draw()
        end
    end"""
    elif ext == ".tvrus":
        return f"""    // Spawning New AI Model
    let config = {lib}.arch.transformer(layers: 12, heads: 8)
    let model = {lib}.spawn_model(config)
    
    // Self-Improvement Loop
    model.train_on_codebase("./src")
    model.optimize(target: "inference_speed")"""
    elif ext == ".shw":
        return f"""    // Absorbing External Knowledge
    let raw_src = {lib}.absorb("https://github.com/external/{topic.replace(' ','_')}")
    
    // Synthesizing Native Library
    let native_lib = {lib}.synthesize(raw_src)
    native_lib.save_as("std::{topic.replace(' ','_').lower()}")"""
    return ""

def get_explanation(ext):
    if ext == ".ss": return "Executes directly on the CPU, bypassing the kernel scheduler."
    if ext == ".hpy": return "Offloads rendering to the GPU while maintaining a non-blocking main thread."
    if ext == ".tvrus": return "Constructs a fresh neural network architecture in memory and initiates training."
    if ext == ".shw": return "Parses foreign ASTs (Abstract Syntax Trees) and recompiles them into native eMo bytecode."
    return ""

if __name__ == "__main__":
    generate_lessons()