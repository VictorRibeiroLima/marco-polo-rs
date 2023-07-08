use serde::{Deserialize, Serialize};

pub trait Paginator {
    type E: std::fmt::Debug + Clone + PartialEq + Eq + Copy + Serialize + for<'a> Deserialize<'a>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaginationOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Pagination<T: Paginator> {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<PaginationOrder>,
    pub order_by: Option<T::E>,
}

impl<T: Paginator> Pagination<T> {
    pub fn new() -> Self {
        Pagination {
            offset: None,
            limit: None,
            order: None,
            order_by: None,
        }
    }
}
