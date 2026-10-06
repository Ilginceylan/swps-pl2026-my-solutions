# Lab 2 — Ownership, Borrowing and Structs
## Move, copy, clone, borrowing, strings, slices, structs

*A quick reference for everything used in Lab 2*

---

# Stack vs heap, in Rust terms

- **Stack**: fixed-size, LIFO, fast — local variables and fixed-size types (`i32`, `bool`, `[T; N]`, a `Fraction`'s two `i32` fields) live here, pushed/popped automatically as scopes enter and exit
- **Heap**: dynamically sized, allocated and freed explicitly — growable or variable-size data (a `String`'s text buffer, a `Vec<T>`'s element buffer) lives here instead
- A `String`/`Vec<T>` *variable* is itself a small, fixed-size stack value — a handle of `(pointer, length, capacity)` — that points at the separate heap buffer
- Moving or copying that handle (3 words) never touches the heap buffer itself — only the handle is duplicated or relocated, which is exactly why heap-owning types can't be `Copy`: two handles would both believe they own, and must free, the same buffer

```rust
let n: i32 = 42;                   // stack only — 4 bytes, size known at compile time
let arr: [i32; 3] = [1, 2, 3];     // stack only — fixed size, no heap involved

let s = String::from("hello");     // stack handle (ptr, len, cap) -> heap buffer "hello"
let v: Vec<i32> = vec![1, 2, 3];   // stack handle (ptr, len, cap) -> heap buffer [1, 2, 3]

struct Account { id: i32, name: String }
let acc = Account { id: 1, name: String::from("Ada") };
// `acc` (the i32 plus the String's 3-word handle) sits on the stack;
// only the text "Ada" lives on the heap

let boxed: Box<i32> = Box::new(42); // forces an otherwise-stack i32 onto the heap
```

---

# Why ownership?

- **C/C++**: manual `free`/`delete` — forget it and you leak, do it twice and you crash (double free), use it after and you get undefined behavior
- **Java/Python**: a garbage collector tracks references at *runtime* and frees automatically — safe, but costs memory overhead and unpredictable pauses
- **Rust**: the compiler tracks ownership **statically**, at compile time, and inserts the free (`drop`) itself — no GC, no manual free, and the three bugs above become compile errors instead of runtime bugs
- This is the same stack/heap picture from the memory lecture — ownership is *who is responsible for freeing this heap allocation*

---

# The three ownership rules

1. Each value has exactly **one owner** at a time
2. When the owner goes out of scope, Rust calls `drop` and frees the value — automatically, deterministically, at a known point
3. Ownership can be **moved** to a new owner, but a value is never implicitly duplicated

These three rules alone rule out use-after-free and double-free, entirely at compile time, with zero runtime checks.

---

# Drop and RAII

- `Drop` is a trait with one method, `fn drop(&mut self)`, called automatically when a value's owner goes out of scope — nobody calls it manually
- This pattern — tie a resource's lifetime to a value's scope — is called **RAII** (Resource Acquisition Is Initialization). It covers more than memory: file handles, sockets, and locks all clean up the same way
- Values drop in reverse declaration order, and each value drops exactly once — the compiler proves this statically, which is exactly why rule 3 (no implicit duplication) has to hold

```rust
struct Logger { name: String }

impl Drop for Logger {
    fn drop(&mut self) {
        println!("closing {}", self.name);
    }
}

{
    let log = Logger { name: String::from("session") };
    // ... use log ...
}   // "closing session" printed here, automatically
```

---

# Move semantics

Assigning or passing a non-`Copy` value transfers ownership — the old binding is no longer usable.

If assignment *copied* a `String`'s heap buffer pointer instead of moving it, both `ticket` and `claimed` would later call `drop` on the **same** heap allocation — a double free. Moving invalidates the old binding so only one owner ever drops it.

```rust
let x = 5; let y = x;
println!("{x} {y}");          // ok — i32 is Copy, both still valid

let ticket = String::from("A-102");
let claimed = ticket;
println!("{ticket}");         // E0382: value moved into claimed

fn process(order: Vec<i32>) -> usize { order.len() }
let orders = vec![1, 2, 3];
process(orders);
println!("{orders:?}");        // E0382: orders moved into process
```

A move is a shallow, compile-time bookkeeping operation — no heap data is copied, only the *right to free it* changes hands.

---

# Copy vs Clone

`Copy` types duplicate on assignment for free; anything owning heap data needs an explicit `.clone()`.

- `Copy` is only implemented for types where a bitwise duplicate is always safe — stack-only data with no custom `Drop` (integers, `bool`, `char`, and tuples/arrays built only from `Copy` types)
- A type that owns a heap allocation **cannot** be `Copy`, because two independent owners would both try to free the same memory — `Clone` makes the duplication explicit and lets it allocate a *second*, independent buffer
- `Copy` and `Move` are mutually exclusive: a type is either copied implicitly on every assignment, or moved

```rust
let price = 20;
let total = price;           // ok — i32 is Copy, no move happens

let title = String::from("draft-report");
let backup = title.clone();  // explicit deep copy
println!("{title} {backup}"); // ok — both still valid, independent buffers
```

Rule of thumb: stack-only, fixed-size data is `Copy`; heap-owning data needs `Clone`.

---

# Borrowing: the rules

Any number of `&T` at once, or exactly one `&mut T` — never both — checked entirely at compile time.

- This is the **aliasing XOR mutability** principle: data is either shared by many readers, or exclusively held by one writer, never both
- It is what gives Rust **memory safety and data-race freedom without a garbage collector or a runtime lock** — the compiler proves it once, before the program ever runs
- A reference is just a pointer plus a compile-time-tracked region of validity — borrowing never copies or moves the owned value, only lends access to it temporarily; the original owner is still the one that eventually `drop`s it

```rust
let mut inventory = vec!["apple", "banana"];
let item = &inventory[0];
inventory.push("cherry");    // E0502: push needs &mut inventory, item still borrows it
println!("{item}");           // push could reallocate and invalidate `item`

let mut counter = 0;
let r1 = &mut counter;
let r2 = &mut counter;        // E0499: two &mut borrows at once
*r1 += 1;

// what IS allowed:

let name = String::from("Ada");
let a = &name;
let b = &name;                 // ok — any number of shared borrows at once
println!("{a} {b}");

let mut score = 10;
let peek = &score;
println!("{peek}");             // peek's last use ends here
let bump = &mut score;          // ok — no immutable borrow still alive
*bump += 1;

fn read_both(x: &i32, y: &i32) -> i32 { x + y }
let p = 3; let q = 4;
read_both(&p, &q);               // ok — each parameter is its own independent borrow

fn add_one(x: &mut i32) { *x += 1; }
let mut m = 5;
add_one(&mut m);
add_one(&mut m);                 // ok — sequential &mut borrows, never overlapping
```

---

# `&self` vs `&mut self`

Mutability is part of the reference's type — a plain `&T` cannot be passed where `&mut T` is required.

- `self`, `&self` and `&mut self` are just syntactic sugar for a first parameter of type `Self`, `&Self` or `&mut Self` — the same borrowing rules apply to it as to any other reference
- A method that only reads its receiver should take `&self`; one that mutates needs `&mut self`; one that consumes/transforms ownership takes `self` by value
- The signature *is* the contract: callers and the compiler both know whether a call can mutate, just by reading `&` vs `&mut` vs nothing

```rust
fn enqueue(q: &Vec<i32>, job: i32) { q.push(job); }     // E0596: q is not &mut

fn enqueue(q: &mut Vec<i32>, job: i32) { q.push(job); } // ok
```

---

# Ownership and function signatures

A function's parameter types tell you exactly what happens to the caller's variable — no need to read the body.

| Parameter type | Caller's variable afterward | Use when |
|---|---|---|
| `T` (by value) | moved (or copied, if `T: Copy`) | the function needs to own or consume it |
| `&T` | still fully usable | the function only needs to read it |
| `&mut T` | usable again once the borrow ends | the function needs to mutate it in place |

This is why Rust function signatures double as documentation: `fn f(v: &mut Vec<i32>)` already tells you `f` mutates `v` and the caller keeps it afterward.

---

# Strings and slices

`String` owns heap-allocated UTF-8 data; `&str` is a borrowed view into it — prefer `&str` parameters.

- A `String` is really `(pointer, length, capacity)` pointing at a heap buffer it owns and will `drop`; `&str` is `(pointer, length)` — no ownership, no capacity, just a read-only window
- Both are guaranteed valid **UTF-8**, so slicing by a byte index that lands mid-character panics — indexing by `[i]` directly is not allowed at all; use `.chars()` to iterate by Unicode scalar value
- A slice is a *view*, so it can only be as long-lived as the data it points into — this is lifetimes again, applied to text

```rust
fn len_in_chars(s: &str) -> usize {
    s.chars().count()
}
len_in_chars("hello");          // 5

let owned = String::from("rust");
len_in_chars(&owned);           // &String coerces to &str
len_in_chars("literal");        // a literal is already &str
```

`&str` accepts both `&String` (via deref coercion) and string literals, so it's the more flexible parameter type.

---

# Vectors and slices

`&[T]` and `&mut [T]` are views into a `Vec<T>` or array — they let a function work without owning the data.

- `Vec<T>` is `(pointer, length, capacity)` on the heap, growable; `[T; N]` is a fixed-size block, usually on the stack; a slice `&[T]` erases that difference down to `(pointer, length)`
- Because a slice doesn't care whether the data came from a `Vec`, an array, or part of one, one function written against `&[T]` works for all three — this is why "prefer slices over `&Vec<T>` in signatures" is idiomatic
- `&mut [T]` lets a function mutate elements **in place** through a borrow, without taking ownership or returning a new collection

```rust
fn sum(v: &[i32]) -> i32 {
    v.iter().sum()
}
sum(&[1, 2, 3, 4]);          // 10

fn increment_all(v: &mut [i32]) {
    for x in v.iter_mut() { *x += 1; }
}

let mut nums = vec![1, 2, 3, 4, 5];
nums.retain(|x| x % 2 == 0);   // keep elements matching a predicate, in place
// nums == [2, 4]
```

---

# Structs and methods

`struct` groups named fields; `impl` adds associated functions (`Self::new`) and methods (`&self`/`&mut self`).

- A struct's fields are laid out together in memory, one after another (plus padding for alignment) — no per-field heap allocation unless a field itself owns heap data (like a `String`)
- `impl` blocks don't change layout at all; they just attach functions to the type. A **method** takes some form of `self`; an **associated function** (like `new`) doesn't, and is called via `Type::name(...)`
- `#[derive(...)]` asks the compiler to *generate* a trait implementation at compile time — zero runtime cost, equivalent to writing the `impl Debug for Fraction { ... }` by hand

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Fraction { num: i32, den: i32 }

impl Fraction {
    const ONE: Fraction = Fraction { num: 1, den: 1 };   // associated constant

    fn new(num: i32, den: i32) -> Self { Fraction { num, den } }  // associated function, no self

    fn value(&self) -> f64 {                          // method, reads self
        self.num as f64 / self.den as f64
    }

    fn flip(&mut self) {                                // method, mutates self
        std::mem::swap(&mut self.num, &mut self.den);
    }
}

let f = Fraction::new(3, 4);
f.value();   // 0.75
```

The `derive` list gives `{:?}` printing, `.clone()`, copy-on-assign, and `==` — remove `Copy` and assignment starts moving instead.

---

# Struct variants

- **Named-field struct** (what Lab 2 uses): `struct Fraction { num: i32, den: i32 }` — fields accessed by name
- **Tuple struct**: fields accessed by position — a lightweight "newtype" that gives a plain type its own identity
- **Unit-like struct**: no fields at all — just a type, often used as a marker or to hang a trait implementation on

```rust
struct Meters(f64);              // tuple struct
let distance = Meters(5.2);
println!("{}", distance.0);      // 5.2

struct Marker;                   // unit-like struct
```

---

# Struct update syntax

`..other` fills in every field you didn't set explicitly, copying or moving them from another value of the same type.

```rust
#[derive(Debug, Clone, Copy)]
struct Config { width: u32, height: u32, fullscreen: bool }

let default_cfg = Config { width: 800, height: 600, fullscreen: false };
let custom = Config { fullscreen: true, ..default_cfg };
// custom.width == 800, custom.height == 600, custom.fullscreen == true
```
