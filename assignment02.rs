// Assignment 2 - Vectors, structs, owning data
// I chose 10 of the 15 mini tasks:
//   Task 4: double_in_place, largest, task4_vectors
//   Task 5: area, contains, can_hold, scale
//   Task 6: Book::new, Library::new, add

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Task 4 - Vectors and slices
// ---------------------------------------------------------------------------

// Given in lab02.rs: running_sum(&[1, 2, 3, 4]) == vec![1, 3, 6, 10].
fn running_sum(values: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(values.len());
    let mut acc = 0;
    for &v in values {
        acc += v;
        result.push(acc);
    }
    result
}

// Given in lab02.rs: [3, 1, 3, 2, 1] -> [3, 1, 2].
fn remove_duplicates(values: &mut Vec<i32>) {
    let mut result = Vec::new();
    for &v in values.iter() {
        if !result.contains(&v) {
            result.push(v);
        }
    }
    *values = result;
}

// Task 4.1: multiply every element by 2, in place.
fn double_in_place(values: &mut [i32]) {
    for i in 0..values.len() {
        values[i] = values[i] * 2;
    }
}

// Task 4.2: largest(&[4, 9, 2]) == Some(9), largest(&[]) == None.
fn largest(values: &[i32]) -> Option<i32> {
    if values.is_empty() {
        return None;
    }
    let mut max = values[0];
    for i in 0..values.len() {
        if values[i] > max {
            max = values[i];
        }
    }
    Some(max)
}

// Task 4.3: remove_duplicates, then double_in_place, then print.
fn task4_vectors() {
    let mut v = vec![3, 1, 3, 2, 1];
    remove_duplicates(&mut v);
    double_in_place(&mut v);
    println!("{:?}", v);
    println!("{:?}", running_sum(&v));
    println!("{:?}", largest(&v));

    // Answer: &v has the type &Vec<i32>, but the functions expect &[i32].
    // This works because of deref coercion: Vec<i32> implements Deref with
    // the target [i32], so Rust automatically turns &Vec<i32> into &[i32].
}

// ---------------------------------------------------------------------------
// Task 5 - Structs and methods
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(top_left: Point, width: f64, height: f64) -> Self {
        Self {
            top_left,
            width,
            height,
        }
    }

    // Associated function, not a method.
    fn square(top_left: Point, base: f64) -> Self {
        Self {
            top_left,
            width: base,
            height: base,
        }
    }

    // Task 5.1: width * height.
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // Task 5.2: is p inside the rectangle (edges included)?
    // y grows downwards, so the bottom edge is top_left.y + height.
    fn contains(&self, p: Point) -> bool {
        let right = self.top_left.x + self.width;
        let bottom = self.top_left.y + self.height;
        p.x >= self.top_left.x && p.x <= right && p.y >= self.top_left.y && p.y <= bottom
    }

    // Task 5.3: does `other` fit inside self? Sizes only, positions ignored.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // Task 5.4: multiply width and height by factor, in place.
    fn scale(&mut self, factor: f64) {
        self.width = self.width * factor;
        self.height = self.height * factor;
    }
}

// My own small test for the Task 5 methods.
fn task5_rectangles() {
    let mut r = Rectangle::new(Point::new(0.0, 0.0), 4.0, 3.0);
    let small = Rectangle::new(Point::new(1.0, 1.0), 2.0, 2.0);
    println!("area = {}", r.area());
    println!("{}", r.contains(Point::new(2.0, 1.0)));
    println!("{}", r.contains(Point::new(5.0, 1.0)));
    println!("{}", r.can_hold(&small));
    println!("{}", small.can_hold(&r));
    r.scale(2.0);
    println!("{:?}", r);
}

// ---------------------------------------------------------------------------
// Task 6 - Owning data in structs: a library
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    year: u16,
}

impl Book {
    // Task 6.1: build a Book that owns its strings.
    fn new(title: &str, author: &str, year: u16) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year: year,
        }
    }
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    // Task 6.2: an empty library.
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    // Task 6.3: the library TAKES OWNERSHIP of the book.
    fn add(&mut self, book: Book) {
        self.books.push(book);
    }
}

// My own small test for the Task 6 functions.
fn task6_check() {
    let mut lib = Library::new();
    lib.add(Book::new("Dune", "Frank Herbert", 1965));
    lib.add(Book::new("Emma", "Jane Austen", 1815));
    println!("{}", lib.books.len());
    println!("{:?}", lib.books[0]);
}

fn main() {
    task4_vectors();
    task5_rectangles();
    task6_check();
}
