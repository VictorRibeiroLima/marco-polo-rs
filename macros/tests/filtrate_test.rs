#![allow(dead_code)]

use marco_polo_rs_macros::Filtrate;

use crate::database::queries::filter::FilterableOptions;

mod database;

#[derive(Filtrate)]
struct Test {
    pub a: i32,
    pub b: String,
    pub c: Option<String>,
    pub d: Option<i32>,
}

#[test]
fn test_basic_struct_fields() {
    let test = InternalFiltrationTestFilters::default();
    assert!(test.a.is_none());
    assert!(test.b.is_none());
    assert!(test.c.is_none());
    assert!(test.d.is_none());
}

#[test]
fn test_empty_where_string() {
    let test = InternalFiltrationTestFilters::default();
    let (where_string, param_count) = test.gen_where_statements(None);
    assert_eq!(where_string, String::new());
    assert_eq!(param_count, 0);
}
