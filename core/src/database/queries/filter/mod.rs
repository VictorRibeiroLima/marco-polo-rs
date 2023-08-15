use std::marker::PhantomData;

use sqlx::{
    postgres::PgArguments,
    query::{Query, QueryAs},
    Postgres,
};

use serde::{Deserialize, Serialize};

mod helper;

pub fn filtration_from_str<'de, D, S>(deserializer: D) -> Result<Option<S>, D::Error>
where
    D: serde::Deserializer<'de>,
    S: std::str::FromStr,
    <S as std::str::FromStr>::Err: std::fmt::Display,
{
    let s = deserializer.deserialize_str(helper::Helper(PhantomData))?;

    Ok(Some(s))
}

pub fn filtration_from_str_option<'de, D, S>(deserializer: D) -> Result<Option<Option<S>>, D::Error>
where
    D: serde::Deserializer<'de>,
    S: std::str::FromStr,
    <S as std::str::FromStr>::Err: std::fmt::Display,
{
    let v = filtration_from_str(deserializer)?;
    match v {
        Some(v) => Ok(Some(Some(v))),
        None => Ok(Some(None)),
    }
}

pub trait FilterableOptions {
    fn apply<O>(
        self,
        query: QueryAs<'_, Postgres, O, PgArguments>,
    ) -> QueryAs<'_, Postgres, O, PgArguments>;

    fn apply_raw(self, query: Query<'_, Postgres, PgArguments>)
        -> Query<'_, Postgres, PgArguments>;
    fn filter_fields(&self) -> Vec<&'static str>;

    fn gen_where_statements(&self, param_count: Option<usize>) -> (String, usize);
}

pub trait Filterable {
    type F: FilterableOptions + Serialize + for<'a> Deserialize<'a> + Default;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filter<T: Filterable> {
    #[serde(flatten)]
    pub options: T::F,
}

impl<T: Filterable> Filter<T> {
    /// Applies the filter to the query in the same order as the fields returned by
    /// `filter_fields`. calling the 'bind' method of query_as.
    pub fn apply<'q>(
        self,
        query: QueryAs<'q, Postgres, T, PgArguments>,
    ) -> QueryAs<'q, Postgres, T, PgArguments> {
        self.options.apply(query)
    }

    /// Applies the filter to the query in the same order as the fields returned by
    /// `filter_fields` our 'gen_where_statements'. calling the 'bind' method of query.
    pub fn apply_raw<'q>(
        self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> Query<'q, Postgres, PgArguments> {
        self.options.apply_raw(query)
    }

    /// Returns the fields that are used by the filter.
    pub fn filter_fields(&self) -> Vec<&'static str> {
        self.options.filter_fields()
    }

    /// Returns the where statement that are used by the filter and the number of parameters used.
    /// The number of parameters is used to calculate the offset for the bind method.
    /// You can pass out the number of parameters of your query to this method,if none is passed 0 is used.
    pub fn gen_where_statements(&self, param_count: Option<usize>) -> (String, usize) {
        self.options.gen_where_statements(param_count)
    }

    /// Returns the where statement that are used by the filter and the number of parameters used.
    /// The where statement is prefixed with the alias.
    /// The number of parameters is used to calculate the offset for the bind method.
    /// You can pass out the number of parameters of your query to this method,if none is passed 0 is used.
    pub fn gen_where_statements_with_alias(
        &self,
        alias: &str,
        param_count: Option<usize>,
    ) -> (String, usize) {
        let (original_query, param_count) = self.gen_where_statements(param_count);

        let mut symbols: Vec<String> = original_query.split(" ").map(|s| s.to_string()).collect();

        for i in 0..symbols.len() {
            let symbol = &symbols[i];
            if symbol == "="
                || symbol == "!="
                || symbol == "<="
                || symbol == ">="
                || symbol == "<"
                || symbol == ">"
                || symbol == "LIKE"
                || symbol == "BETWEEN"
                || symbol == "IS"
            {
                symbols[i - 1] = format!("{}.{}", alias, symbols[i - 1]);
            }
        }

        let where_statements = symbols.join(" ");

        return (where_statements, param_count);
    }
}

impl<T: Filterable> Default for Filter<T> {
    fn default() -> Self {
        Self {
            options: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDateTime;
    use marco_polo_rs_macros::Filtrate;

    use super::Filter;

    #[derive(Filtrate, Default)]
    #[allow(dead_code)]
    struct Foo {
        pub a: String,
        pub b: i32,
        pub c: Option<String>,
        pub d: NaiveDateTime,
        pub e: Option<NaiveDateTime>,
    }

    #[test]
    fn test_where_with_alias() {
        let mut filter: Filter<Foo> = Default::default();
        filter.options.a = Some("a".to_string());
        filter.options.b = Some(1);
        filter.options.c = Some(None);
        filter.options.d = Some(NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
        filter.options.e = Some(Some(NaiveDateTime::from_timestamp_opt(0, 0)).unwrap());

        let (where_statements, _) = filter.gen_where_statements_with_alias("v", None);

        assert_eq!(
            where_statements,
            "v.a LIKE $1 AND v.b = $2 AND v.c IS NULL AND v.d = $3 AND v.e = $4"
        );
    }
}
