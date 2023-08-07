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
