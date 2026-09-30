//! Lab 1 — First Steps in Rust — exercises.
//!
//! Fill in each `todo!()` with a working implementation. Everything here
//! compiles as-is (a stub); running it will panic on the first `todo!()`
//! reached until you implement it. Full task descriptions: lab01-rust-basics.md.

#![allow(dead_code, unused_variables, unused_mut)]

// ---------------------------------------------------------------------------
// Task 1 — Formatting output
// ---------------------------------------------------------------------------
// Using ONE println! per line, print exactly:
//   Name: Ada, age: 36
//   Pi to 3 places: 3.142
//   |   right|left    | center |
//   Debug: (1, "two", 3.0)
//   0x ff, 0b 1010
// Hints: {}, {:?}, {:.3}, {:>8}, {:<8}, {:^8}, {:x}, {:b}. You can write
// variable names inline: println!("{name}").

fn task1_formatting() {
    let name = "Ada";
    let age = 31;
    println!("Name {name}, age {age}");
    println!("Pi to 3 places: {:.3}", std::f64::consts::PI);
    println!("|{:>8}|{:<8}|{:^8}|", "right", "left", "center");
    println!("{:?}", (1, "two", 3.0));
}

// ---------------------------------------------------------------------------
// Task 2 — Variables, mutability and shadowing
// ---------------------------------------------------------------------------
fn task2_variables() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner: {x}");
    }
    println!("outer: {x}");

    let mut count = 0;
    count += 1;
    println!("{count}");
}

// ---------------------------------------------------------------------------
// Task 3 — Scalar types and overflow
// ---------------------------------------------------------------------------
// factorial: return None if the result would not fit in a u64. Use
// checked_mul in a loop from 2..=n. Option is covered properly in Lab 3 —
// for now, Some(v) means "there is a value", None means "there is none".
//   factorial(0) == Some(1), factorial(5) == Some(120),
//   factorial(20) == Some(2432902008176640000), factorial(21) == None.
fn factorial(n: i32) -> Option<u64> {
    if n >= 0 {
        let mut acc: u64 = 1;
        for i in 2..=n as u64 {
            acc *= i;
        }
        Some(acc)
    } else {
        None
    }
}

// task3_scalars:
// 1. Print i8::MIN, i8::MAX, u8::MAX, i32::MAX, u64::MAX, f64::EPSILON.
// 2. As a comment: what happens with `let x: u8 = 255; let y = x + 1;` in a
//    debug build vs. a release build (`cargo run --release`)? Why?
// 3. For `250u8 + 10`, print the result of checked_add, wrapping_add,
//    saturating_add and overflowing_add.
// 4. Print factorial(20) and factorial(21) using {:?} (they're Options).
// 5. Casting with `as`: print what `300_i32 as u8`, `-1_i32 as u32`,
//    `3.99_f64 as i32` and `-3.99_f64 as u32` give, plus `u8::try_from(300)`.
//    As a comment: why is `as` considered dangerous, and what's the safer
//    alternative?
fn task3_scalars() {
    println!("{:?}", factorial(20));
    println!("{:?}", factorial(-10));
}

// ---------------------------------------------------------------------------
// Task 4 — Functions and expressions
// ---------------------------------------------------------------------------
// Almost everything in Rust is an expression. A block's value is its last
// expression, written WITHOUT a trailing semicolon.

// Write the body without using the `return` keyword.
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// Write the body without using the `return` keyword.
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Return "negative", "zero" or "positive" depending on x. Use `if` as an
// expression (no `return`).
fn sign(x: i32) -> String {
    if x > 0 {
        "positive".to_string()
    } else if x == 0 {
        "zero".to_string()
    } else {
        "negative".to_string()
    }
}

// This one-liner currently would fail to compile if written as
// `x * x;` (a statement, value `()`) instead of `x * x` (an expression).
// Write it as an expression so it actually returns i32.
fn square(x: i32) -> i32 {
    todo!()
}

// task4_expressions:
// Call celsius_to_fahrenheit(100.0), fahrenheit_to_celsius(212.0),
// sign(-3) and square(7), printing the results.
// Then: use a block expression to compute a value, e.g.
//   let y = { let a = 3; a * a + 1 };
// and print y. As a comment: what would y be if the block's last line ended
// with a semicolon instead?
fn task4_expressions() {
    todo!()
}

// ---------------------------------------------------------------------------
// Task 5 — Control flow
// ---------------------------------------------------------------------------
// Each function below practises a different construct. Implement all of
// them, then have task5_control_flow print FizzBuzz for 1..=20.

// Use `match` on the tuple (n % 3, n % 5).
// 3 -> "Fizz", 10 -> "Buzz", 15 -> "FizzBuzz", 7 -> "7"
fn fizzbuzz(n: u32) -> String {
    todo!()
}

// Use `while`. 1 -> 0, 6 -> 8, 27 -> 111.
fn collatz_steps(mut n: u64) -> u32 {
    todo!()
}

// Use `loop` or `while` — Euclid's algorithm. gcd(48, 18) == 6.
fn gcd(mut a: u64, mut b: u64) -> u64 {
    todo!()
}

// Use `for` over a range with an early `return`.
// is_prime(2) == true, is_prime(97) == true, is_prime(1) == false.
fn is_prime(n: u64) -> bool {
    todo!()
}

// Use `for` with tuple assignment: (a, b) = (b, a + b).
// fib(0) == 0, fib(10) == 55, fib(50) == 12586269025.
fn fib(n: u32) -> u64 {
    todo!()
}

// Use `match` with inclusive ranges, e.g. 90..=100.
// 95 -> 'A', 72 -> 'C', 30 -> 'F'. Cover every possible u32 value: what
// happens if you remove one arm?
fn grade(points: u32) -> char {
    todo!()
}

// Print fizzbuzz(1..=20) with a `for` loop, then call the functions above
// with the example values from their comments and print the results.
// Also try: for i in (1..=5).rev() {...} and for i in (0..20).step_by(5) {...}.
// As a comment: what's the difference between 1..10 and 1..=10?
fn task5_control_flow() {
    todo!()
}

// ---------------------------------------------------------------------------
// Task 6 — loop with a value, labelled loops
// ---------------------------------------------------------------------------

// `loop` can return a value via `break value`.
// first_power_of_two_above(5) == 8, first_power_of_two_above(8) == 16.
fn first_power_of_two_above(n: u64) -> u64 {
    todo!()
}

// Print this triangle with two nested `for` loops (outer i in 1..=9, inner
// j in 1..=9). Stop each row as soon as i * j > limit, using `continue
// 'outer` on a labelled outer loop:
//   1 2 3 4 5 6 7 8 9
//   2 4 6 8 10 12
//   3 6 9 12
//   4 8 12
//   5 10
//   6 12
//   7
//   8
//   9
fn task6_labelled(limit: u32) {
    todo!()
}

// ---------------------------------------------------------------------------
// Task 7 — Tuples and arrays
// ---------------------------------------------------------------------------

// Return (min, max, average) of values.
// min_max_avg(vec![3, -1, 7, 2]) == (-1, 7, 2.75)
fn min_max_avg(values: Vec<i32>) -> (i32, i32, f64) {
    todo!()
}

// Swap the two elements of the tuple, returning them in the opposite order
// and with types swapped accordingly.
fn swap_pair(p: (i32, String)) -> (String, i32) {
    todo!()
}

// 1. Call min_max_avg with vec![3, -1, 7, 2] and destructure + print the
//    result: let (lo, hi, avg) = ...;
// 2. Create `let mut grid = [[0u8; 3]; 3];`, set the diagonal to 1 with a
//    plain indexed loop (for i in 0..3 { grid[i][i] = 1; }), print with {:?}.
// 3. As a comment: what happens if you index grid[5][0]? Compile error or
//    runtime panic? Try once with a constant index, once with an index
//    computed at runtime (e.g. read from a variable).
// 4. Call swap_pair and print the result.
fn task7_compound() {
    todo!()
}

pub fn run() {
    task1_formatting();
    task2_variables();
    task3_scalars();
    task4_expressions();
    task5_control_flow();
    println!(
        "first power of two above 5: {}",
        first_power_of_two_above(5)
    );
    task6_labelled(12);
    task7_compound();
}
