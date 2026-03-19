# Sorting Library — Rust

A generic, plug-and-play sorting library designed for any data type that implements the `PartialOrd` trait. This project was developed as a semester-end project for the Rust Programming course.

## Features
* **Generic Implementation**: Works with integers, floats, strings, or custom structs.
* **Trait-Based**: Leverages Rust's powerful trait system for clean API calls.
* **Multiple Algorithms**: Choose the best algorithm for your specific use case.

## Algorithms
- **Bubble Sort**
- **Insertion Sort**
- **Selection Sort**
- **Merge Sort**
- **Quick Sort**

## Usage

Add the library to your logic and call the `sort` method on your collection:

```rust
use sorter::{BubbleSort, SortOrder, Sorter};

fn main() {
    let mut numbers = vec![5, 3, 8, 1];
    
    // Sorting in Ascending order
    BubbleSort.sort(&mut numbers, SortOrder::Ascending);
    
    assert_eq!(numbers, vec![1, 3, 5, 8]);
}
CommandsUse the following commands to run the demo, execute tests, or view the documentation:ActionCommandRun Democargo run --bin sorter_demoRun Testscargo testView Docscargo doc --openCourse: Semester-end project — Rust Programming
### Git Commands
To push these changes, run these in your terminal:

```bash
git add README.md
git commit -m "docs: improve README structure and syntax"
git push
