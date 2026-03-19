use crate::traits::{SortOrder, Sorter};

pub struct BubbleSort;

impl<T: PartialOrd> Sorter<T> for BubbleSort {
    fn name(&self) -> &'static str {
        "Bubble Sort"
    }

    fn sort(&self, data: &mut Vec<T>, order: SortOrder) {
        let n = data.len();
        for i in 0..n {
            for j in 0..n - 1 - i {
                let should_swap = match order {
                    SortOrder::Ascending => data[j] > data[j + 1],
                    SortOrder::Descending => data[j] < data[j + 1],
                };
                if should_swap {
                    data.swap(j, j + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending() {
        let mut v = vec![5, 3, 8, 1, 9, 2];
        BubbleSort.sort(&mut v, SortOrder::Ascending);
        assert_eq!(v, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_descending() {
        let mut v = vec![5, 3, 8];
        BubbleSort.sort(&mut v, SortOrder::Descending);
        assert_eq!(v, vec![8, 5, 3]);
    }

    #[test]
    fn test_strings() {
        let mut v = vec!["banana", "apple", "cherry"];
        BubbleSort.sort(&mut v, SortOrder::Ascending);
        assert_eq!(v, vec!["apple", "banana", "cherry"]);
    }
}
