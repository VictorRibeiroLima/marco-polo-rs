use sqlx::{
    postgres::PgArguments,
    query::{Query, QueryAs},
    Postgres,
};

use serde::{Deserialize, Serialize};

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

    /// Returns the where statements that are used by the filter and the number of parameters used.
    /// The number of parameters is used to calculate the offset for the bind method.
    /// You can pass out the number of parameters of your query to this method,if none is passed 0 is used.
    pub fn gen_where_statements(&self, param_count: Option<usize>) -> (String, usize) {
        self.options.gen_where_statements(param_count)
    }
}

impl<T: Filterable> Default for Filter<T> {
    fn default() -> Self {
        Self {
            options: Default::default(),
        }
    }
}
