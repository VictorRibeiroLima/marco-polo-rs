macro_rules! find_all {
    ($table:ty,$table_name:expr) => {
        pub async fn find_all(
            pool: &sqlx::PgPool,
            pagination: crate::database::queries::pagination::Pagination<$table>,
            filter: crate::database::queries::filter::Filter<$table>,
        ) -> Result<Vec<$table>, sqlx::Error> {
            let (offset, limit, order, order_by) = pagination.to_tuple();

            let (where_sql, param_count) = filter.gen_where_statements(None);

            let sql;
            if where_sql.is_empty(){
                sql = format!(
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
            }else {
                sql = format!(
                    r#"
                    SELECT 
                        *
                    FROM 
                        {} 
                    WHERE
                        {}
                    ORDER BY 
                        {} {}
                    LIMIT
                        ${}
                    OFFSET 
                        ${}
                    "#,
                    $table_name,
                    where_sql,
                    order_by.name(),
                    order.name(),
                    param_count + 1,
                    param_count + 2,
                );
            }

            let mut query = sqlx::query_as(&sql);
            query = filter.apply(query);
            query = query.bind(limit).bind(offset);

            let result: Vec<$table> = query
                .fetch_all(pool)
                .await?;

            return Ok(result);
        }
    };
}

pub(crate) use find_all;
