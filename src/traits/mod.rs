#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

pub trait Sorter<T: PartialOrd> {
    fn sort(&self, data: &mut Vec<T>, order: SortOrder);
    fn name(&self) -> &'static str;
}
