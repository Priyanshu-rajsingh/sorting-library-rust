use crate::traits::{SortOrder, Sorter};

pub struct SelectionSort;

impl<T: PartialOrd> Sorter<T> for SelectionSort {
    fn name(&self) -> &'static str {
        "Selection Sort"
    }

    fn sort(&self, data: &mut Vec<T>, order: SortOrder) {
        let n = data.len();
        for i in 0..n {
            let mut best = i;
            for j in i + 1..n {
                let is_better = match order {
                    SortOrder::Ascending => data[j] < data[best],
                    SortOrder::Descending => data[j] > data[best],
                };
                if is_better {
                    best = j;
                }
            }
            data.swap(i, best);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending() {
        let mut v = vec![3, 1, 4, 1, 5];
        SelectionSort.sort(&mut v, SortOrder::Ascending);
        assert_eq!(v, vec![1, 1, 3, 4, 5]);
    }
}
