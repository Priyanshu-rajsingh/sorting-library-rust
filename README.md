# Sorting Library — Rust

A generic plug-and-play sorting library that works on any data type
implementing the `PartialOrd` trait.

## Algorithms
- Bubble Sort
- Insertion Sort
- Selection Sort
- Merge Sort
- Quick Sort

## Usage
```rust
use sorter::{BubbleSort, SortOrder, Sorter};

let mut numbers = vec![5, 3, 8, 1];
BubbleSort.sort(&mut numbers, SortOrder::Ascending);
```

## Run
```
cargo run --bin sorter_demo
cargo test
cargo doc --open
```

```

Then push it:
```
git add README.md
git commit -m "Add README"
git push
