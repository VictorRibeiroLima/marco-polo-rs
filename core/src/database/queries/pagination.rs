use serde::{Deserialize, Serialize};

pub trait Paginator {
    type E: std::fmt::Debug
        + Clone
        + PartialEq
        + Default
        + Eq
        + Copy
        + Serialize
        + for<'a> Deserialize<'a>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum PaginationOrder {
    #[default]
    Asc,
    Desc,
}

impl PaginationOrder {
    pub fn name(&self) -> String {
        match self {
            PaginationOrder::Asc => "ASC".to_string(),
            PaginationOrder::Desc => "DESC".to_string(),
        }
    }
}

const DEFAULT_LIMIT: i64 = 10;
const DEFAULT_OFFSET: i64 = 0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    /// Converts the Pagination struct into a tuple of (offset, limit, order, order_by)
    pub fn to_tuple(self) -> (i64, i64, PaginationOrder, T::E) {
        (
            self.offset.unwrap_or(DEFAULT_OFFSET),
            self.limit.unwrap_or(DEFAULT_LIMIT),
            self.order.unwrap_or_default(),
            self.order_by.unwrap_or_default(),
        )
    }
}

impl<T: Paginator> Default for Pagination<T> {
    fn default() -> Self {
        Pagination {
            offset: Some(DEFAULT_OFFSET),
            limit: Some(DEFAULT_LIMIT),
            order: Some(PaginationOrder::default()),
            order_by: Some(T::E::default()),
        }
    }
}
