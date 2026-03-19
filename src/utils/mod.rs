use crate::traits::SortOrder;

pub fn is_sorted<T: PartialOrd>(data: &[T], order: SortOrder) -> bool {
    for i in 0..data.len() - 1 {
        match order {
            SortOrder::Ascending => {
                if data[i] > data[i + 1] {
                    return false;
                }
            }
            SortOrder::Descending => {
                if data[i] < data[i + 1] {
                    return false;
                }
            }
        }
    }
    true
}
