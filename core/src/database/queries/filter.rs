use sqlx::{
    postgres::PgArguments,
    query::{Query, QueryAs},
    Postgres,
};

trait InnerBind {}

pub trait FilterableOptions {
    fn apply<O>(
        self,
        query: QueryAs<'_, Postgres, O, PgArguments>,
    ) -> QueryAs<'_, Postgres, O, PgArguments>;

    fn apply_raw(self, query: Query<'_, Postgres, PgArguments>)
        -> Query<'_, Postgres, PgArguments>;
    fn filter_fields(&self) -> Vec<&'static str>;
}

pub trait Filterable {
    type F: FilterableOptions;
}

pub struct Filter<T: Filterable> {
    option: T::F,
}

impl<T: Filterable> Filter<T> {
    pub fn apply<'q>(
        self,
        query: QueryAs<'q, Postgres, T, PgArguments>,
    ) -> QueryAs<'q, Postgres, T, PgArguments> {
        self.option.apply(query)
    }

    pub fn apply_raw<'q>(
        self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> Query<'q, Postgres, PgArguments> {
        self.option.apply_raw(query)
    }

    pub fn filter_fields(&self) -> Vec<&'static str> {
        self.option.filter_fields()
    }
}
