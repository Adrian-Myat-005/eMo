# 💧 HappyCry Language Reference (v0.6.5)

> "Write like a human, compile like a machine."

HappyCry (`.hpy`) is a domain-specific language that compiles directly to high-performance **Rust**. The V2 Compiler features a professional Lexer/Parser pipeline, structured AST, and optimized code generation.

---

## 1. Syntax Basics

### Comments
- **Line Comment**: `# This is a comment` (or `##`)
- **Block Comment**: 
  ```hpy
  #_ 
    This is a 
    multi-line comment 
  _#
  ```

### Variables & Output
- **Set Variable**: `set my_var to 42`
- **Arithmetic**: `math my_var add 5` (Supports `add`, `sub`, `mul`, `div` in future, currently `add` trigger)
- **Expressions**: Supports parentheses and precedence: `set result to (10 + 5) * 2`
- **Print to Console**: `say "Hello World"`
- **CLI Arguments**: `set target to input arg "--target"` (Gets value after flag)

---

## 2. Control Flow

### Loops
Iterate a specific number of times.
```hpy
loop 5 times do
    say "Looping..."
end
```

### Conditionals
Compare variables against values (string or numeric).
```hpy
if name is "Adrian" do
    say "Access Granted"
end
```

---

## 3. HappyWeb (Full Stack)

Build web applications with a built-in Actix-Web server and compile-time HTML rendering.

### Server Initialization
Define the server port at the top level.
```hpy
server new on port 8080
```

### Page Definitions
Define UI routes using `page` blocks.
```hpy
page "/" do
    title "Dashboard"
    style theme dark
    add header "Welcome Admin"
    add button "Run Scan" triggers "/scan"
end
```

### Starting the Server
The `serve` command at the end of the file triggers the blocking server loop.
```hpy
serve
```

---

## 4. SecOps (Async & Networking)

HappyCry is built for network operations. All tasks run on the **Tokio** runtime.

### Async Tasks
Define background workflows.
```hpy
async task port_scan do
    say "Starting scan..."
    try connect "192.168.1.1" on 80 do
        say "Port 80 is OPEN"
    end
end
```

### Execution
- **Await**: `await port_scan` (Waits for task to finish)
- **HTTP**: `say http get "https://api.ipify.org"` (Returns body text)
- **TCP Check**: `try connect "IP" on PORT do ... end` (Executes block on success)

---

## 5. Swarm Mode (Distributed P2P)

Create decentralized applications where nodes find each other automatically using **libp2p** (Gossipsub + mDNS).

### Joining the Swarm
Join a specific topic channel.
```hpy
swarm join "happy-net-v1"
```

### Listening for Events
Trigger code when a specific message is received from the swarm.
```hpy
on swarm message "PING" do
    say "Ping received!"
    swarm broadcast "PONG"
end
```

### Broadcasting
Send a message to all other peers.
```hpy
swarm broadcast "HELLO_WORLD"
```

---

## 6. HappyBase (Database)

Built-in SQLite support for persistence.

```hpy
db open "system.db"
db run "CREATE TABLE IF NOT EXISTS logs (id INTEGER PRIMARY KEY, msg TEXT)"
db insert into "logs" values "(NULL, 'System started')"
```

---

## 7. Universal Bridge (FFI)

Link against C libraries directly.

```hpy
# Link to a system library
link library "m"

# Define the foreign function signature
foreign fn cos(float) -> float

# Call it (unsafe block generated automatically)
call cos(0.0)
```

---

## 8. The Escape Hatch (Raw Rust)

Inject Rust code verbatim.

```rust
#happy-raw
fn complex_calculation(x: i32) -> i32 {
    x * x + 2
}
let val = complex_calculation(5);
println!("Verbatim Rust: {}", val);
#happy-end
```

---

## 10. AI & Automation (The "Brain")

HappyCry V2 integrates directly with the **ThinkingVirus (`tvrus`)** and the **Shadow System**.

### AI Logic in Code
You can embed AI prompts directly into your logic.
```hpy
# Ask the AI to process data at runtime
set result to ai think "Summarize this log error" about error_log_var
```

### The Shadow System (`shadow`)
Learn from existing repositories and clone them into HappyCry templates.
1. `shadow learn --url <git_url>`: Analyzes a repo and creates a blueprint.
2. `shadow clone <blueprint.shw>`: Generates a `main.hpy` scaffold.

### The ThinkingVirus (`tvrus`)
Your AI companion for development.
- **Generate**: `tvrus vibe "Create a secure login system"`
- **Edit**: `tvrus edit main.hpy "Add a logout route"`

---

## 11. Compiler Pipeline

1.  **Lex**: Raw text is converted to a stream of `Tokens`.
2.  **Parse**: Tokens are structured into an Abstract Syntax Tree (`AST`).
3.  **Codegen**: The AST is traversed to generate optimized Rust source code.
4.  **Build**: The `cargo` toolchain compiles the generated Rust into a native, standalone binary.