// Fixed by borrowing: changed the parameter to &String and the call to &a, so ownership of 'a' stays in main() and it can still be used afterward.
fn print_len(s: &String) { println!("{}", s.len()); }
fn task_a() {
    let a = String::from("hello");
    print_len(&a);
    println!("{a}");
}

// Fixed by reordering: push(4) now happens before borrowing &v[0].
// Before, 'first' held an immutable borrow while push() needed a mutable borrow at the same time, which Rust forbids.
// Now the borrow starts only after the vector has already been modified, so there's no conflict.
fn task_b() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    let first = &v[0];
    println!("first = {first}");
}

// Fixed by borrowing: shout() now takes &str instead of String, and the call passes &name.
// Ownership of 'name' stays in main(), so the earlier borrow 'r' is still valid when it is printed.
fn shout(s: &str) -> String { s.to_uppercase() }

fn task_f() {
    let name = String::from("ada");
    let r = &name;
    let loud = shout(&name);
    println!("{r} {loud}");
}

// Fixed by collecting first: the loop over &scores holds an immutable borrow, so pushing into scores inside it is forbidden.
// Now matching values go into a separate vector 'extra', and are added to scores with extend() after the loop ends.
fn task_g() {
    let mut scores = vec![10, 20, 30];
    let mut extra = Vec::new(); 
    for s in &scores {
        if *s > 15 {
            extra.push(*s);
        }
    }scores.extend(extra);
    println!("{scores:?}");
}
// Fixed by calling c.bump() directly, one call at a time: the two &mut borrows 'a' and 'b' were alive at the same time, which Rust forbids.
// Now each mutable borrow ends right after its call, so they never overlap.
struct Counter { value: i32 }
impl Counter {
    fn bump(&mut self) -> i32 {
        self.value += 1;
        self.value
    }
}
fn task_h() {
    let mut c = Counter { value: 0 };
    let a = c.bump();
    let b = c.bump();
    println!("{} {}", a, b);
}

// Fixed by returning an owned String instead of a reference: 'w' is a local variable that is dropped when the function ends,
// so a reference into it would dangle. Now the String is moved out of 'w' and ownership passes to the caller.

struct Wrapper { value: String }

fn unwrap_text(w: &Wrapper) -> &str {
    &w.value
}

fn broken_unwrap() -> String {
    let w = Wrapper { value: String::from("hi") };
    w.value
}

fn task_i() {
    println!("{}", broken_unwrap());
}

// Part 2 - 1
fn first_word(s: &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[..i];
        }
    }
    s
}

fn task_p2_1() {
    println!("{}", first_word("hello world"));
    println!("{}", first_word("single"));
}

// Part 2 - 2
fn count_vowels(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        let small = c.to_ascii_lowercase();
        if small == 'a' || small == 'e' || small == 'i' || small == 'o' || small == 'u' {
            count += 1;
        }
    }
    count
}

fn task_p2_2() {
    println!("{}", count_vowels("Hello World"));
}

// Part 2 - 3
fn reverse_words(s: &str) -> String {
    let mut result = String::new();
    for word in s.split_whitespace().rev() {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(word);
    }
    result
}

fn task_p2_3() {
    println!("{}", reverse_words("the quick brown fox"));
}

// Part 2 - 4 \ Uppercases the first character and leaves the rest unchanged.
fn capitalize(s: &str) -> String {
    let mut result = String::new();
    let mut first = true;
    for c in s.chars() {
        if first {
            result.push(c.to_ascii_uppercase());
            first = false;
        } else {
            result.push(c);
        }
    }
    result
}

fn task_p2_4() {
    println!("{}", capitalize("rust"));
    println!("[{}]", capitalize(""));
}

fn main() {
    task_a();
    task_b();
    task_f();
    task_g();
    task_h();
    task_i();
    task_p2_1();
    task_p2_2();
    task_p2_3();
    task_p2_4();
}