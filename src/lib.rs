pub mod algorithms;
pub mod error;
pub mod traits;
pub mod utils;

pub use algorithms::{BubbleSort, InsertionSort, MergeSort, QuickSort, SelectionSort};
pub use error::SortError;
pub use traits::{SortOrder, Sorter};

pub fn sort_with<T, S>(sorter: &S, data: &mut Vec<T>, order: SortOrder)
where
    T: PartialOrd,
    S: Sorter<T>,
{
    sorter.sort(data, order);
}
