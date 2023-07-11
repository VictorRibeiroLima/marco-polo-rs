macro_rules! test_find_all {
    ($table:ty,$enum:ty,$find_all:ident,$fixture_name:expr) => {
        #[sqlx::test(migrations = "../migrations", fixtures($fixture_name))]
        async fn test_find_all(pool: sqlx::PgPool) {
            let pagination = crate::database::queries::pagination::Pagination::default();

            let result = $find_all(&pool, pagination).await;
            let list = result.unwrap();
            let list_len = list.len();

            assert_eq!(list_len, 10)
        }

        #[sqlx::test(migrations = "../migrations", fixtures($fixture_name))]
        async fn test_find_all_limit_20(pool: sqlx::PgPool) {
            let pagination = crate::database::queries::pagination::Pagination {
                offset: None,
                limit: Some(20),
                order_by: None,
                order: None,
            };
            let result = $find_all(&pool, pagination).await;
            let list = result.unwrap();
            let list_len = list.len();

            assert_eq!(list_len, 20)
        }

        #[sqlx::test(migrations = "../migrations", fixtures($fixture_name))]
        async fn test_find_all_order_by_id_desc(pool: sqlx::PgPool) {
            let pagination = crate::database::queries::pagination::Pagination {
                offset: None,
                limit: None,
                order_by: Some(<$enum>::Id),
                order: Some(crate::database::queries::pagination::PaginationOrder::Desc),
            };
            let result = $find_all(&pool, pagination).await;
            let list = result.unwrap();
            let list_len = list.len();

            assert_eq!(list_len, 10);
            let mut base_id: i32 = 0;
            for (index, member) in list.into_iter().enumerate() {
                if index == 0 {
                    base_id = member.id;
                } else {
                    assert!(member.id < base_id);
                    base_id = member.id;
                }
            }
        }

        #[sqlx::test(migrations = "../migrations", fixtures($fixture_name))]
        async fn test_find_all_order_by_id_asc(pool: sqlx::PgPool) {
            let pagination = crate::database::queries::pagination::Pagination {
                offset: None,
                limit: None,
                order_by: Some(<$enum>::Id),
                order: Some(crate::database::queries::pagination::PaginationOrder::Asc),
            };
            let result = $find_all(&pool, pagination).await;
            let list = result.unwrap();
            let list_len = list.len();

            assert_eq!(list_len, 10);
            let mut base_id: i32 = 0;
            for (index, member) in list.into_iter().enumerate() {
                if index == 0 {
                    base_id = member.id;
                } else {
                    assert!(member.id > base_id);
                    base_id = member.id;
                }
            }
        }

        #[sqlx::test(migrations = "../migrations", fixtures($fixture_name))]
        async fn test_offset(pool: sqlx::PgPool) {
            let pagination = crate::database::queries::pagination::Pagination {
                offset: None,
                limit: None,
                order_by: Some(<$enum>::Id),
                order: Some(crate::database::queries::pagination::PaginationOrder::Asc),
            };

            let mut result = $find_all(&pool, pagination).await.unwrap();
            let last_member = result.pop().unwrap();

            let pagination = crate::database::queries::pagination::Pagination {
                offset: Some(10),
                limit: None,
                order_by: Some(<$enum>::Id),
                order: Some(crate::database::queries::pagination::PaginationOrder::Asc),
            };

            let result = $find_all(&pool, pagination).await.unwrap();
            let member = &result[0];

            assert_eq!(member.id, last_member.id + 1)
        }
    };
}

pub(crate) use test_find_all;
