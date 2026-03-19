use crate::traits::{SortOrder, Sorter};

pub struct InsertionSort;

impl<T: PartialOrd> Sorter<T> for InsertionSort {
    fn name(&self) -> &'static str {
        "Insertion Sort"
    }

    fn sort(&self, data: &mut Vec<T>, order: SortOrder) {
        let n = data.len();
        for i in 1..n {
            let mut j = i;
            while j > 0 {
                let should_swap = match order {
                    SortOrder::Ascending => data[j - 1] > data[j],
                    SortOrder::Descending => data[j - 1] < data[j],
                };
                if should_swap {
                    data.swap(j - 1, j);
                    j -= 1;
                } else {
                    break;
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
        let mut v = vec![4, 2, 7, 1];
        InsertionSort.sort(&mut v, SortOrder::Ascending);
        assert_eq!(v, vec![1, 2, 4, 7]);
    }
}
