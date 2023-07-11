macro_rules! find_all {
    ($table:ty,$enum_default:expr,$table_name:expr) => {
        pub async fn find_all(
            pool: &sqlx::PgPool,
            pagination: crate::database::queries::pagination::Pagination<$table>,
        ) -> Result<Vec<$table>, sqlx::Error> {
            let (offset, limit, order, order_by) = pagination.to_tuple();

            let sql = format!(
                r#"
                SELECT 
                    *
                FROM 
                    {} 
                ORDER BY 
                    {} {}
                LIMIT
                    $1
                OFFSET 
                    $2
                "#,
                $table_name,
                order_by.name(),
                order.name()
            );

            let result: Vec<$table> = sqlx::query_as(&sql)
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?;

            return Ok(result);
        }
    };
}

pub(crate) use find_all;
