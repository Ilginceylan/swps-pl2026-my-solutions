# Lab 1 — Rust Basics
## Cargo, variables, types, functions, control flow

*A quick reference for everything used in Lab 1*

---

# Cargo

Cargo manages a whole project — dependencies, build, test — not just one file like `rustc`.

```bash
cargo new myproject && cd myproject
cargo run              # build + run
cargo check            # fast, no codegen
cargo build --release  # optimized
cargo fmt               cargo clippy               cargo test
```

---

# Formatting output

`{}` uses `Display`, `{:?}` uses `Debug`; width/precision/base go inside the `{}`.

```rust
println!("{name}, age {age}");        // Bob, age 42
println!("{:.2}", 2.71828);           // 2.72
println!("{:>6}|{:<6}|{:^6}", "a", "b", "c");
println!("{:?}", (1, "two", 3.0));    // (1, "two", 3.0)
println!("{:x} {:b}", 255, 5);        // ff 101
```

---

# Variables: mutability and shadowing

`let` is immutable by default; `mut` allows reassignment; shadowing rebinds the same name, even to a new type.

```rust
let x = 5;
// x = 6;          // error: cannot assign twice
let mut y = 5;
y = 6;              // ok

let z = 1;
let z = z + 1;      // shadow: z is now 2, a fresh binding

let value = "7";
let value: i32 = value.parse().unwrap();  // shadow: &str -> i32

const LIMIT: u32 = 1000;   // compile-time constant, never mut
```

---

# Scalar types and overflow

Debug builds panic on overflow, release builds wrap silently — Rust makes the choice explicit via these methods.

```rust
let x: u8 = 200;
x.checked_add(100);      // None
x.wrapping_add(100);     // 44
x.saturating_add(100);   // 255
x.overflowing_add(100);  // (44, true)

300_i32 as u8;            // 44  (as truncates silently)
-1_i32 as u32;            // 4294967295
u8::try_from(300_i32);    // Err(...)  — the safe, checked version
```

---

# Functions and expressions

Rust is expression-oriented: a block's value is its last expression, with no trailing `;` and no `return` needed.

```rust
fn double(x: i32) -> i32 {
    x * 2   // tail expression = return value, no `return`
}

fn describe(x: i32) -> &'static str {
    if x < 0 { "negative" } else { "non-negative" }   // if as an expression
}

fn broken(x: i32) -> i32 {
    x + 1;   // the `;` makes the block's value `()` — type mismatch, won't compile
}

let y = { let a = 3; a + 1 };   // block expression: y == 4
```

---

# Control flow

`match` must cover every case (exhaustiveness is checked at compile time); it can match tuples and ranges.

```rust
let pair = (1, -1);
match pair {
    (0, 0) => println!("both zero"),
    (0, _) | (_, 0) => println!("one zero"),
    _ => println!("neither"),
}

let temp_c = 18;
let weather = match temp_c {
    30..=50 => "hot",    // inclusive range
    10..=29 => "mild",
    _       => "cold",
};

for i in (1..=5).rev() {}        // 5 4 3 2 1
for i in (0..20).step_by(5) {}   // 0 5 10 15
```

---

# `loop` with a value, labelled loops

`loop` is an expression — `break value` gives it a result; labels let `continue`/`break` target an outer loop.

```rust
let mut count = 0;
let result = loop {
    count += 1;
    if count == 10 { break count * 2; }   // loop's value
};
// result == 20

'outer: for i in 0..5 {
    for j in 0..5 {
        if j == 2 { continue 'outer; }   // skip straight to next i
        print!("{i}-{j} ");
    }
}
```

---

# Tuples and arrays

Tuples are fixed-size and heterogeneous; arrays are fixed-size and homogeneous, with the length part of the type.

```rust
let pair: (i32, &str) = (1, "one");
let (n, word) = pair;              // destructuring

let a: [i32; 4] = [10, 20, 30, 40];
let m = [[0u8; 2]; 2];              // 2x2 array of arrays

a[1];   // ok, in bounds
a[9];   // panics at runtime — bounds are always checked, just not always at compile time
```
