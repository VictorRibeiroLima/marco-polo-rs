pub trait FromRows: Sized {
    fn from_rows(rows: &Vec<sqlx::postgres::PgRow>) -> Result<Vec<Self>, sqlx::Error>;
}

pub trait FromRowAlias: Sized {
    fn from_row_alias(row: &sqlx::postgres::PgRow, alias: &str) -> Result<Self, sqlx::Error>;
}
