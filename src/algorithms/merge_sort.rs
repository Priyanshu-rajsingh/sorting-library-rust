use crate::traits::{SortOrder, Sorter};

pub struct MergeSort;

impl<T: PartialOrd + Clone> Sorter<T> for MergeSort {
    fn name(&self) -> &'static str {
        "Merge Sort"
    }

    fn sort(&self, data: &mut Vec<T>, order: SortOrder) {
        let len = data.len();
        if len <= 1 {
            return;
        }
        let mid = len / 2;
        let mut left = data[..mid].to_vec();
        let mut right = data[mid..].to_vec();
        self.sort(&mut left, order);
        self.sort(&mut right, order);
        merge(data, left, right, order);
    }
}

// fn merge<T: PartialOrd + Clone>(
//     data: &mut [T], left: Vec<T>, right: Vec<T>, order: SortOrder,

fn merge<T: PartialOrd + Clone>(data: &mut [T], left: Vec<T>, right: Vec<T>, order: SortOrder) {
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        let pick_left = match order {
            SortOrder::Ascending => left[i] <= right[j],
            SortOrder::Descending => left[i] >= right[j],
        };
        if pick_left {
            data[k] = left[i].clone();
            i += 1;
        } else {
            data[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        data[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        data[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending() {
        let mut v = vec![6, 2, 9, 1];
        MergeSort.sort(&mut v, SortOrder::Ascending);
        assert_eq!(v, vec![1, 2, 6, 9]);
    }
}
