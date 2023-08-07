use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgArguments,
    query::{Query, QueryAs},
    Postgres,
};

use std::marker::PhantomData;

use serde::de::Error;
use serde::de::Visitor;

pub struct Helper<S>(pub PhantomData<S>);

//impl
impl<'de, S> Visitor<'de> for Helper<S>
where
    S: std::str::FromStr,
    <S as std::str::FromStr>::Err: std::fmt::Display,
{
    type Value = S;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        value.parse::<Self::Value>().map_err(Error::custom)
    }
}

#[allow(dead_code)]
pub fn filtration_from_str<'de, D, S>(deserializer: D) -> Result<Option<S>, D::Error>
where
    D: serde::Deserializer<'de>,
    S: std::str::FromStr,
    <S as std::str::FromStr>::Err: std::fmt::Display,
{
    let s = deserializer.deserialize_str(Helper(PhantomData))?;

    Ok(Some(s))
}

#[allow(dead_code)]
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
