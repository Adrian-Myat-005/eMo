import os
import random

def generate_syntax_lessons():
    base_dir = 'language_lessons'
    os.makedirs(base_dir, exist_ok=True)
    
    # Curriculum: 1000+ Syntax Lessons
    curriculum = [
        # TIER 1: NOVICE (Variables & Types)
        (0, 200, "1_Novice_Syntax", [
            ("Variable Declaration", "set x to 10\nlet name = \"eMo\""),
            ("Constants", "const PI = 3.14159"),
            ("Integers", "let count: int = 42"),
            ("Floats", "let price: float = 19.99"),
            ("Booleans", "let is_active = true"),
            ("Strings", "let greeting = \"Hello Universe\""),
            ("String Interpolation", "log(\"Count is {count}\")"),
            ("Basic Math", "let sum = 10 + 5"),
            ("Comments", "// Single line\n/* Block */"),
            ("Printing", "log(\"Output\")")
        ]),
        
        # TIER 2: BEGINNER (Control Flow)
        (200, 400, "2_Beginner_Control_Flow", [
            ("If Statement", "if x > 10 { log(\"Big\") }"),
            ("If Else", "if x > 0 { log(\"Pos\") } else { log(\"Neg\") }"),
            ("Match Expression", "match status { \"OK\" => log(1), _ => log(0) }"),
            ("Loop (Infinite)", "loop { log(\"Forever\") }"),
            ("While Loop", "while x > 0 { x = x - 1 }"),
            ("For Loop (Range)", "for i in 0..10 { log(i) }"),
            ("For Loop (Iter)", "for item in list { process(item) }"),
            ("Break/Continue", "if x == 5 { break }"),
            ("Functions", "fn add(a, b) { return a + b }"),
            ("Void Functions", "fn do_work() { log(\"Work\") }")
        ]),
        
        # TIER 3: INTERMEDIATE (Data Structures)
        (400, 600, "3_Intermediate_Data", [
            ("Struct Definition", "struct User { name: str, age: int }"),
            ("Struct Instantiation", "let u = User { name: \"Neo\", age: 30 }"),
            ("Enums", "enum Color { Red, Green, Blue }"),
            ("Arrays (Fixed)", "let nums: [int; 3] = [1, 2, 3]"),
            ("Vectors (Dynamic)", "let list = vec![1, 2, 3]"),
            ("HashMaps", "let map = { \"key\": \"value\" }"),
            ("Tuples", "let pair = (10, \"Ten\")"),
            ("Option Type", "let maybe: Option<int> = Some(5)"),
            ("Result Type", "fn try_it() -> Result<int, err>"),
            ("Methods", "impl User { fn say_hi() { log(self.name) } }")
        ]),
        
        # TIER 4: ADVANCED (Memory & Generics)
        (600, 800, "4_Advanced_Architecture", [
            ("Ownership (Move)", "let a = b // b is moved"),
            ("Borrowing (Ref)", "fn read(data: &str)"),
            ("Mutable Borrow", "fn update(data: &mut int)"),
            ("Lifetimes", "fn longest<'a>(x: &'a str) -> &'a str"),
            ("Generic Functions", "fn id<T>(x: T) -> T { x }"),
            ("Generic Structs", "struct Box<T> { inner: T }"),
            ("Traits (Interfaces)", "trait Fly { fn fly(&self) }"),
            ("Impl Trait", "impl Fly for Bird { fn fly() {} }"),
            ("Closures (Lambdas)", "let add = |a, b| a + b"),
            ("Smart Pointers", "let heap_val = Box::new(10)")
        ]),
        
        # TIER 5: MASTER (Meta & Concurrency)
        (800, 1005, "5_Master_Metaprogramming", [
            ("Macros", "macro_rules! say_hello { () => { log(\"Hi\") } }"),
            ("Attributes", "#[derive(Debug)] struct Point"),
            ("Thread Spawning", "thread::spawn(|| { work() })"),
            ("Channels", "let (tx, rx) = channel()") ,
            ("Async/Await", "async fn fetch() { .await }"),
            ("Unsafe Blocks", "unsafe { raw_ptr.write(0) }"),
            ("FFI (C Binding)", "extern \"C\" { fn c_func(); }"),
            ("Operator Overload", "impl Add for Vector2"),
            ("Custom Iterators", "impl Iterator for Sequence"),
            ("Reflection", "let type_name = reflect::type_name::<T>()")
        ])
    ]

    print("Generating 1005 Language Syntax Lessons...")

    for start, end, folder_name, topics_list in curriculum:
        folder_path = os.path.join(base_dir, folder_name)
        os.makedirs(folder_path, exist_ok=True)
        
        for i in range(start, end):
            lesson_num = i + 1
            # Cycle through topics for this tier
            topic_name, topic_code = topics_list[i % len(topics_list)]
            
            filename = os.path.join(folder_path, f"syntax_{lesson_num:04d}.md")
            
            content = f"""# Syntax Lesson {lesson_num:04d}: {topic_name}
## Tier: {folder_name.replace("_", " ")}

### 1. The Syntax Rule
In **eMo**, {topic_name.lower()} is fundamental to expressing logic.

```emo
// Example: {topic_name}
{topic_code}
```

### 2. Breakdown
*   **Keyword/Symbol:** Identify the key tokens used above.
*   **Behavior:** How the compiler interprets this structure.
*   **Usage:** Used in `.ss`, `.tvrus`, and `.shw` (eMo Core). **Note:** `.hpy` (HappyCry) uses a simplified dialect (`set`, `do`, `end`).

### 3. Drill
1.  Create a file named `drill_{lesson_num}.emo`.
2.  Write a function that uses **{topic_name}**.
3.  Compile it to ensure you understand the syntax.

---
*eMo Language Syntax Course*
"""
            with open(filename, 'w') as f:
                f.write(content)

if __name__ == "__main__":
    generate_syntax_lessons()
