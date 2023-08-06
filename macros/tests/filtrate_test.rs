#![allow(dead_code)]

use marco_polo_rs_macros::Filtrate;
use sqlx::{Encode, Postgres, Type};

use crate::database::queries::filter::FilterableOptions;

use serde::{Deserialize, Serialize};

mod database;

#[derive(PartialEq, Clone, Deserialize, Serialize, Debug, Copy)]
struct TestDate {}

impl<T> Encode<'_, T> for TestDate
where
    T: sqlx::database::Database,
{
    fn encode_by_ref(
        &self,
        _buf: &mut <T as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        sqlx::encode::IsNull::No
    }
}

impl Type<Postgres> for TestDate {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("date")
    }
}

#[derive(Filtrate)]
struct Test {
    pub a: i32,
    pub b: String,
    pub c: Option<String>,
    pub d: Option<i32>,
}

#[derive(Filtrate)]
struct Test2 {
    pub a: i32,
    pub b: String,
    #[filtrate(name = "foo")]
    pub c: Option<String>,
    #[filtrate(skip = true)]
    pub d: Option<i32>,
}

#[derive(Filtrate)]
struct Test3WithDate {
    pub a: i32,
    pub b: String,
    pub c: Option<String>,
    pub d: Option<i32>,
    pub e: TestDate,
    pub f: Option<TestDate>,
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
fn test_rename_struct() {
    let test = InternalFiltrationTest2Filters::default();
    assert!(test.c.is_none());
}

#[test]
fn test_rename_where() {
    let mut test = InternalFiltrationTest2Filters::default();
    test.c = Some(Some(String::from("test")));

    let (where_string, param_count) = test.gen_where_statements(None);

    assert_eq!(where_string, String::from("foo LIKE $1"));
    assert_eq!(param_count, 1);
}

#[test]
fn test_empty_where_string() {
    let test = InternalFiltrationTestFilters::default();
    let (where_string, param_count) = test.gen_where_statements(None);
    assert_eq!(where_string, String::new());
    assert_eq!(param_count, 0);
}

#[test]
fn test_null_option_where() {
    let test = InternalFiltrationTestFilters {
        a: None,
        b: None,
        c: None,
        d: Some(None),
    };

    let (where_string, param_count) = test.gen_where_statements(None);

    assert_eq!(where_string, String::from("d IS NULL"));
    assert_eq!(param_count, 0);
}

#[test]
fn test_null_2_option_where() {
    let test = InternalFiltrationTestFilters {
        a: None,
        b: None,
        c: Some(None),
        d: Some(None),
    };

    let (where_string, param_count) = test.gen_where_statements(None);

    assert_eq!(where_string, String::from("c IS NULL AND d IS NULL"));
    assert_eq!(param_count, 0);
}

#[test]
fn test_full_query() {
    let test = InternalFiltrationTestFilters {
        a: Some(1),
        b: Some(String::from("test")),
        c: Some(Some(String::from("test"))),
        d: Some(Some(1)),
    };

    let (where_string, param_count) = test.gen_where_statements(None);

    assert_eq!(
        where_string,
        String::from("a = $1 AND b LIKE $2 AND c LIKE $3 AND d = $4")
    );
    assert_eq!(param_count, 4);
}

#[test]
fn test_full_query_with_null() {
    let test = InternalFiltrationTestFilters {
        a: Some(1),
        b: Some(String::from("test")),
        c: Some(None),
        d: Some(None),
    };

    let (where_string, param_count) = test.gen_where_statements(None);

    assert_eq!(
        where_string,
        String::from("a = $1 AND b LIKE $2 AND c IS NULL AND d IS NULL")
    );
    assert_eq!(param_count, 2);
}

#[test]
fn test_full_query_with_null_and_date() {
    let test = InternalFiltrationTest3WithDateFilters {
        a: Some(1),
        b: Some(String::from("test")),
        c: Some(None),
        d: Some(None),
        e: Some(TestDate {}),
        f: Some(None),
        e_end: None,
        f_end: None,
        e_start: None,
        f_start: None,
    };

    let (where_string, param_count) = test.gen_where_statements(None);

    assert_eq!(
        where_string,
        String::from("a = $1 AND b LIKE $2 AND c IS NULL AND d IS NULL AND e = $3 AND f IS NULL")
    );
    assert_eq!(param_count, 3);
}

#[test]
fn test_between_date() {
    let test = InternalFiltrationTest3WithDateFilters {
        a: Some(1),
        b: Some(String::from("test")),
        c: Some(None),
        d: Some(None),
        e: None,
        f: Some(None),
        e_end: Some(TestDate {}),
        f_end: Some(None),
        e_start: Some(TestDate {}),
        f_start: Some(None),
    };

    let (where_string, param_count) = test.gen_where_statements(None);

    assert_eq!(
        where_string,
        String::from("a = $1 AND b LIKE $2 AND c IS NULL AND d IS NULL AND e BETWEEN $3 AND $4 AND f IS NULL")
    );
    assert_eq!(param_count, 4);
}

#[test]
fn test_start_some_value_end_some_none_date() {
    let test = InternalFiltrationTest3WithDateFilters {
        a: Some(1),
        b: Some(String::from("test")),
        c: Some(None),
        d: Some(None),
        e: None,
        f: None,
        e_end: None,
        f_end: Some(None),
        e_start: None,
        f_start: Some(Some(TestDate {})),
    };

    let (where_string, param_count) = test.gen_where_statements(None);

    assert_eq!(
        where_string,
        String::from("a = $1 AND b LIKE $2 AND c IS NULL AND d IS NULL AND f >= $3")
    );
    assert_eq!(param_count, 3);
}

#[test]
fn test_end_some_none_end_some_value_date() {
    let test = InternalFiltrationTest3WithDateFilters {
        a: Some(1),
        b: Some(String::from("test")),
        c: Some(None),
        d: Some(None),
        e: None,
        f: None,
        e_end: None,
        f_end: Some(Some(TestDate {})),
        e_start: None,
        f_start: None,
    };

    let (where_string, param_count) = test.gen_where_statements(None);

    assert_eq!(
        where_string,
        String::from("a = $1 AND b LIKE $2 AND c IS NULL AND d IS NULL AND f <= $3")
    );
    assert_eq!(param_count, 3);
}
