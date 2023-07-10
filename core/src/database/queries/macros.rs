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

macro_rules! test_find_all {
    ($table:ty,$find_all:ident,$fixture_name:expr) => {
        #[sqlx::test(migrations = "../migrations", fixtures($fixture_name))]
        async fn test_find_all(pool: Pool<sqlx::Postgres>) {
            let pagination = crate::database::queries::pagination::Pagination {
                offset: None,
                limit: None,
                order_by: None,
                order: None,
            };
            let result = $find_all(&pool, pagination).await;
            let list = result.unwrap();
            let list_len = list.len();

            assert_eq!(list_len, 10)
        }
    };
}

pub(crate) use find_all;
pub(crate) use test_find_all;
