use sorter::{sort_with, SortOrder, Sorter};
use sorter::{BubbleSort, InsertionSort, MergeSort, QuickSort, SelectionSort};

// #[derive(Debug, PartialEq, PartialOrd)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Student {
    gpa: f64,
    name: String,
}

fn main() {
    println!("=== Plug-and-Play Sorting Library Demo ===\n");

    let mut numbers = vec![42, 7, 19, 3, 88, 25];
    println!("Original : {:?}", numbers);
    sort_with(&BubbleSort, &mut numbers, SortOrder::Ascending);
    println!("Bubble  ↑: {:?}", numbers);
    sort_with(&QuickSort, &mut numbers, SortOrder::Descending);
    println!("Quick   ↓: {:?}\n", numbers);

    // let mut floats = vec![3.14, 1.41, 2.71, 1.73];
    let mut floats = vec![5.5, 1.2, 8.9, 3.3, 2.1];
    InsertionSort.sort(&mut floats, SortOrder::Ascending);
    println!("Floats  ↑: {:?}\n", floats);

    let mut words = vec!["rust", "python", "java", "c", "go"];
    SelectionSort.sort(&mut words, SortOrder::Ascending);
    println!("Words   ↑: {:?}\n", words);

    let mut students = vec![
        Student {
            gpa: 8.5,
            name: "Alice".to_string(),
        },
        Student {
            gpa: 9.2,
            name: "Bob".to_string(),
        },
        Student {
            gpa: 7.8,
            name: "Carol".to_string(),
        },
    ];
    MergeSort.sort(&mut students, SortOrder::Descending);
    println!("Students by GPA (high to low):");
    for s in &students {
        println!("  {} — GPA {}", s.name, s.gpa);
    }

    println!("\nAvailable algorithms:");
    let sorters: Vec<&dyn Sorter<i32>> = vec![
        &BubbleSort,
        &InsertionSort,
        &SelectionSort,
        &MergeSort,
        &QuickSort,
    ];
    for s in &sorters {
        println!("  -> {}", s.name());
    }
}
