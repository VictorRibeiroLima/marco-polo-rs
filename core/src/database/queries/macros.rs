macro_rules! find_all {
    ($table:ty,$enum_default:expr,$table_name:expr) => {
        pub async fn find_all(
            pool: &sqlx::PgPool,
            pagination: crate::database::queries::pagination::Pagination<$table>,
        ) -> Result<Vec<$table>, sqlx::Error> {
            let offset = match pagination.offset {
                Some(offset) => offset,
                None => 0,
            };

            let limit = match pagination.limit {
                Some(limit) => limit,
                None => 10,
            };

            let order = match pagination.order {
                Some(order) => order,
                None => crate::database::queries::pagination::PaginationOrder::Asc,
            };

            let order_by = match pagination.order_by {
                Some(order_by) => order_by,
                None => $enum_default,
            };

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
