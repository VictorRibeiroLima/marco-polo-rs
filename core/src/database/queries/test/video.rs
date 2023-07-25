use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::{
    models::{
        video::{Video, VideoFilters, VideoOrderFields},
        video_storage::StorageVideoStage,
    },
    queries::{
        filter::FilterableOptions,
        video::{
            create, find_all, find_by_id, find_by_id_with_storage, find_by_transcription_id,
            CreateVideoDto,
        },
    },
};

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn filtration_test_id_url(pool: sqlx::PgPool) {
    let mut filters = VideoFilters::default();
    filters.id = Some(uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap());
    filters.url = Some(Some(String::from(
        "https://www.youtube.com/watch?v=1234567890",
    )));

    let mut query = String::from("SELECT * FROM videos WHERE ");

    for (index, filter) in filters.filter_fields().iter().enumerate() {
        if index == 0 {
            query.push_str(&format!("{} = ${}", filter, index + 1));
        } else {
            query.push_str(&format!(" AND {} = ${}", filter, index + 1));
        }
    }

    let mut query = sqlx::query_as(&query);

    query = filters.apply(query);

    let videos: Vec<Video> = query.fetch_all(&pool).await.unwrap();

    assert_eq!(videos.len(), 1);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn filtration_test_id_deleted_at(pool: sqlx::PgPool) {
    let mut filters = VideoFilters::default();

    let date = NaiveDate::from_ymd_opt(2021, 9, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    filters.id = Some(uuid::Uuid::from_str("806b5cdc-f221-11ed-a05b-0242ac120076").unwrap());
    filters.deleted_at = Some(Some(date));

    let mut query = String::from("SELECT * FROM videos WHERE ");

    for (index, filter) in filters.filter_fields().iter().enumerate() {
        if index == 0 {
            query.push_str(&format!("{} = ${}", filter, index + 1));
        } else {
            query.push_str(&format!(" AND {} = ${}", filter, index + 1));
        }
    }

    let mut query = sqlx::query_as(&query);

    query = filters.apply(query);

    let videos: Vec<Video> = query.fetch_all(&pool).await.unwrap();

    assert_eq!(videos.len(), 1);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn filtration_test_id_deleted_at_none(pool: sqlx::PgPool) {
    let mut filters = VideoFilters::default();

    filters.id = Some(uuid::Uuid::from_str("806b5cfc-f221-11ed-a05b-0242ac120075").unwrap());
    filters.deleted_at = Some(None);

    let mut query = String::from("SELECT * FROM videos WHERE ");

    for (index, filter) in filters.filter_fields().iter().enumerate() {
        if index == 0 {
            query.push_str(&format!("{} = ${}", filter, index + 1));
        } else {
            query.push_str(&format!(" AND {} = ${}", filter, index + 1));
        }
    }

    let mut query = sqlx::query_as(&query);

    query = filters.apply(query);

    let videos: Vec<Video> = query.fetch_all(&pool).await.unwrap();

    assert_eq!(videos.len(), 1);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_find_all(pool: sqlx::PgPool) {
    let pagination = crate::database::queries::pagination::Pagination::default();

    let result = find_all(&pool, pagination).await;
    let list = result.unwrap();
    let list_len = list.len();

    assert_eq!(list_len, 10)
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_find_all_limit_20(pool: sqlx::PgPool) {
    let pagination = crate::database::queries::pagination::Pagination {
        offset: None,
        limit: Some(20),
        order_by: None,
        order: None,
    };
    let result = find_all(&pool, pagination).await;
    let list = result.unwrap();
    let list_len = list.len();

    assert_eq!(list_len, 20)
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_find_all_order_by_id_asc(pool: sqlx::PgPool) {
    let pagination = crate::database::queries::pagination::Pagination {
        offset: None,
        limit: None,
        order_by: Some(<VideoOrderFields>::Id),
        order: Some(crate::database::queries::pagination::PaginationOrder::Asc),
    };
    let result = find_all(&pool, pagination).await;
    let list = result.unwrap();
    let list_len = list.len();

    assert_eq!(list_len, 10);
    let mut base_id: Uuid = Uuid::nil();
    for (index, member) in list.into_iter().enumerate() {
        if index == 0 {
            base_id = member.id;
        } else {
            assert!(member.id > base_id);
            base_id = member.id;
        }
    }
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_find_all_order_by_id_desc(pool: sqlx::PgPool) {
    let pagination = crate::database::queries::pagination::Pagination {
        offset: None,
        limit: None,
        order_by: Some(<VideoOrderFields>::Id),
        order: Some(crate::database::queries::pagination::PaginationOrder::Desc),
    };
    let result = find_all(&pool, pagination).await;
    let list = result.unwrap();
    let list_len = list.len();

    assert_eq!(list_len, 10);
    let mut base_id: Uuid = Uuid::nil();
    for (index, member) in list.into_iter().enumerate() {
        if index == 0 {
            base_id = member.id;
        } else {
            assert!(member.id < base_id);
            base_id = member.id;
        }
    }
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_offset(pool: sqlx::PgPool) {
    let pagination = crate::database::queries::pagination::Pagination {
        offset: None,
        limit: Some(11),
        order_by: None,
        order: None,
    };

    let result = find_all(&pool, pagination).await.unwrap();
    let expected_member = &result[10];

    let pagination = crate::database::queries::pagination::Pagination {
        offset: Some(10),
        limit: None,
        order_by: Some(<VideoOrderFields>::Id),
        order: Some(crate::database::queries::pagination::PaginationOrder::Asc),
    };

    let result = find_all(&pool, pagination).await.unwrap();
    let member = &result[0];

    assert_eq!(member.id, expected_member.id)
}

#[sqlx::test(migrations = "../migrations", fixtures("user", "channel"))]
async fn test_create_video(pool: PgPool) {
    let id = uuid::Uuid::new_v4();

    let dto = CreateVideoDto {
        id: &id,
        title: "Test",
        description: "Test",
        user_id: 666,
        channel_id: 666,
        language: "en",
    };

    create(&pool, dto).await.unwrap();

    let count = sqlx::query!("SELECT COUNT(*) FROM videos where id = $1", id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert!(count.count.is_some());
    assert_eq!(count.count.unwrap(), 1);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_create_if_foreign_key(pool: PgPool) {
    let id = uuid::Uuid::new_v4();

    let dto = CreateVideoDto {
        id: &id,
        title: "Test",
        description: "Test",
        user_id: 666,
        channel_id: 666,
        language: "en",
    };

    let result = create(&pool, dto).await;

    assert!(result.is_err());
}
#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_find_by_video_id(pool: PgPool) {
    let id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let find_success = find_by_id(&pool, &id).await.unwrap();

    assert_eq!(find_success.id, id);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_not_find_by_video_id(pool: PgPool) {
    let id = uuid::Uuid::from_str("4fa91b48-f370-11ed-a05b-0242ac120003").unwrap(); //Invalid Uuid for the test
    let find_not_success = find_by_id(&pool, &id).await;

    assert!(find_not_success.is_err());
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("videos", "service_providers", "video_storage")
)]

async fn test_find_by_id_with_storage(pool: PgPool) {
    let id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let video_stage = StorageVideoStage::Raw;
    let storage_id = 1234;

    let find_success = find_by_id_with_storage(&pool, &id, video_stage)
        .await
        .unwrap();

    assert_eq!(find_success.video.id, id);
    assert_eq!(find_success.storage.storage_id, storage_id);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("videos", "service_providers", "video_storage")
)]

async fn test_not_find_by_id_with_storage(pool: PgPool) {
    let id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120095").unwrap(); //Invalid Uuid for the test
    let video_stage = StorageVideoStage::Raw;

    let find_error = find_by_id_with_storage(&pool, &id, video_stage).await;

    assert!(find_error.is_err());
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("videos", "videos_transcriptions")
)]
async fn test_find_by_transcription_id(pool: PgPool) {
    let id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let transcription_id = "Transcription_Test_Ok";

    let find_success = find_by_transcription_id(&pool, transcription_id)
        .await
        .unwrap();

    assert_eq!(find_success.id, id);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("videos", "videos_transcriptions")
)]
async fn test_not_find_by_transcription_id(pool: PgPool) {
    let transcription_id = "Transcription_Test_Err";
    let find_not_success = find_by_transcription_id(&pool, transcription_id).await;

    assert!(find_not_success.is_err());
}
