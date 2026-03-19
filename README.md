# Sorting Library — Rust

A generic plug-and-play sorting library that works on any data type
implementing the `PartialOrd` trait.

## Algorithms
- Bubble Sort
- Insertion Sort
- Selection Sort
- Merge Sort
- Quick Sort

## Supported Data Types
- Integers (i32)
- Floats (f64)
- Strings (&str)
- Custom structs (any type with PartialOrd)

## Project Structure
sorter/
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── algorithms/
│   │   ├── bubble_sort.rs
│   │   ├── insertion_sort.rs
│   │   ├── selection_sort.rs
│   │   ├── merge_sort.rs
│   │   └── quick_sort.rs
│   ├── traits/
│   ├── error/
│   └── utils/
├── tests/
└── benches/

## Run the Demo
cargo run --bin sorter_demo

## Run Tests
cargo test

## Generate Documentation
cargo doc --open

## Course
Semester-end project — Rust Programming
Save it, then push:
git add README.md
git commit -m "Add README"
git push
