use crate::traits::{SortOrder, Sorter};

pub struct QuickSort;

impl<T: PartialOrd> Sorter<T> for QuickSort {
    fn name(&self) -> &'static str {
        "Quick Sort"
    }

    fn sort(&self, data: &mut Vec<T>, order: SortOrder) {
        let len = data.len();
        if len <= 1 {
            return;
        }
        quicksort(data, 0, len - 1, order);
    }
}

fn quicksort<T: PartialOrd>(data: &mut [T], low: usize, high: usize, order: SortOrder) {
    if low < high {
        let pivot = partition(data, low, high, order);
        if pivot > 0 {
            quicksort(data, low, pivot - 1, order);
        }
        quicksort(data, pivot + 1, high, order);
    }
}

fn partition<T: PartialOrd>(data: &mut [T], low: usize, high: usize, order: SortOrder) -> usize {
    let mut i = low;
    for j in low..high {
        let should_move = match order {
            SortOrder::Ascending => data[j] < data[high],
            SortOrder::Descending => data[j] > data[high],
        };
        if should_move {
            data.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, high);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending() {
        let mut v = vec![10, 3, 7, 2, 8];
        QuickSort.sort(&mut v, SortOrder::Ascending);
        assert_eq!(v, vec![2, 3, 7, 8, 10]);
    }
}
