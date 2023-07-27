use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgArguments,
    query::{Query, QueryAs},
    Postgres,
};

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
