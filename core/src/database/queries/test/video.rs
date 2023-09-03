use std::str::FromStr;

use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::{
    models::{
        original_video::OriginalVideo,
        video::{Video, VideoOrderFields},
        video_storage::StorageVideoStage,
    },
    queries::{
        filter::Filter,
        pagination::Pagination,
        video::{
            create, create_errors, create_many, find_all, find_by_id, find_by_id_with_storage,
            find_by_transcription_id,
            with_original::{
                find_all_with_original, find_all_with_original_and_video_channels,
                find_by_user_id_with_original, find_by_user_id_with_original_and_video_channels,
                find_with_original, find_with_original_and_video_channels,
            },
            CreateErrorsDto, CreateVideoDto,
        },
    },
};

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn filtration_test_id(pool: sqlx::PgPool) {
    let mut filter: Filter<Video> = Filter::default();
    filter.options.id = Some(uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap());

    let mut query = String::from("SELECT * FROM videos WHERE ");

    let (where_sql, _) = filter.gen_where_statements(None);

    query.push_str(&where_sql);

    let mut query = sqlx::query_as(&query);

    query = filter.apply(query);

    let videos: Vec<Video> = query.fetch_all(&pool).await.unwrap();

    assert_eq!(videos.len(), 1);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn filtration_test_id_deleted_at(pool: sqlx::PgPool) {
    let mut filter: Filter<Video> = Filter::default();

    let date = NaiveDate::from_ymd_opt(2021, 9, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    filter.options.id = Some(uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120002").unwrap());
    filter.options.deleted_at = Some(Some(date));

    let mut query = String::from("SELECT * FROM videos WHERE ");

    let (where_sql, _) = filter.gen_where_statements(None);

    query.push_str(&where_sql);

    let mut query = sqlx::query_as(&query);

    query = filter.apply(query);

    let videos: Vec<Video> = query.fetch_all(&pool).await.unwrap();

    assert_eq!(videos.len(), 1);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn filtration_test_id_deleted_at_none(pool: sqlx::PgPool) {
    let mut filter: Filter<Video> = Filter::default();

    filter.options.id = Some(uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120001").unwrap());
    filter.options.deleted_at = Some(None);

    let mut query = String::from("SELECT * FROM videos WHERE ");

    let (where_sql, _) = filter.gen_where_statements(None);

    query.push_str(&where_sql);

    let mut query = sqlx::query_as(&query);

    query = filter.apply(query);

    let videos: Vec<Video> = query.fetch_all(&pool).await.unwrap();

    assert_eq!(videos.len(), 1);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_find_all(pool: sqlx::PgPool) {
    let pagination = crate::database::queries::pagination::Pagination::default();
    let filter = crate::database::queries::filter::Filter::default();

    let result = find_all(&pool, pagination, filter).await;
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
    let filter = crate::database::queries::filter::Filter::default();
    let result = find_all(&pool, pagination, filter).await;
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
    let filter = crate::database::queries::filter::Filter::default();
    let result = find_all(&pool, pagination, filter).await;
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
    let filter = crate::database::queries::filter::Filter::default();
    let result = find_all(&pool, pagination, filter).await;
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

    let filter = crate::database::queries::filter::Filter::default();
    let result = find_all(&pool, pagination, filter.clone()).await.unwrap();
    let expected_member = &result[10];

    let pagination = crate::database::queries::pagination::Pagination {
        offset: Some(10),
        limit: None,
        order_by: Some(<VideoOrderFields>::Id),
        order: Some(crate::database::queries::pagination::PaginationOrder::Asc),
    };

    let result = find_all(&pool, pagination, filter).await.unwrap();
    let member = &result[0];

    assert_eq!(member.id, expected_member.id)
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("user", "channel", "original_video")
)]
async fn test_create_video(pool: PgPool) {
    let id = uuid::Uuid::new_v4();

    let channel_ids = vec![666];

    let dto = CreateVideoDto {
        id,
        title: "Test",
        description: "Test",
        user_id: 666,
        channel_ids: &channel_ids,
        language: "en",
        end_time: None,
        original_id: 666,
        start_time: "00:00:00",
        tags: None,
    };

    create(&pool, dto).await.unwrap();

    let count = sqlx::query!("SELECT COUNT(*) FROM videos where id = $1", id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert!(count.count.is_some());
    assert_eq!(count.count.unwrap(), 1);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("user", "channel", "original_video")
)]
async fn test_create_video_with_tags(pool: PgPool) {
    let id = uuid::Uuid::new_v4();

    let channel_ids = vec![666];

    let dto = CreateVideoDto {
        id: id,
        title: "Test",
        description: "Test",
        user_id: 666,
        end_time: None,
        channel_ids: &channel_ids,
        language: "en",
        original_id: 666,
        start_time: "00:00:00",
        tags: Some("test;test".into()),
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
async fn test_create_fail_if_foreign_key(pool: PgPool) {
    let id = uuid::Uuid::new_v4();
    let channel_ids = vec![666];

    let dto = CreateVideoDto {
        id,
        title: "Test",
        description: "Test",
        user_id: 666,
        end_time: None,
        channel_ids: &channel_ids,
        language: "en",
        original_id: 666,
        start_time: "00:00:00",
        tags: None,
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

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_create_error_mark_video_error(pool: PgPool) {
    let video_id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let error = "Test Error";

    let dto = CreateErrorsDto {
        video_ids: vec![video_id],
        error: &error,
    };

    create_errors(&pool, dto).await.unwrap();

    let video = find_by_id(&pool, &video_id).await.unwrap();

    assert!(video.error)
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_create_error_0_previous_errors(pool: PgPool) {
    let video_id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let error = "Test Error";

    let dto = CreateErrorsDto {
        video_ids: vec![video_id],
        error: &error,
    };

    let result = create_errors(&pool, dto).await.unwrap();

    assert_eq!(result, 1);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos_errors"))]
async fn test_create_error_1_previous_errors(pool: PgPool) {
    let video_id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let error = "Test Error";

    let dto = CreateErrorsDto {
        video_ids: vec![video_id],
        error: &error,
    };

    let result = create_errors(&pool, dto).await.unwrap();

    assert_eq!(result, 2);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos_errors"))]
async fn test_create_error_1_previous_errors_from_another_stage(pool: PgPool) {
    let video_id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let error = "Test Error";

    sqlx::query!(
        r#"
        UPDATE videos
        SET stage = 'UPLOADING'
        WHERE id = $1
        "#,
        video_id
    )
    .execute(&pool)
    .await
    .unwrap();

    let dto = CreateErrorsDto {
        video_ids: vec![video_id],
        error: &error,
    };

    let result = create_errors(&pool, dto).await.unwrap();
    assert_eq!(result, 1);
}

#[sqlx::test(migrations = "../migrations", fixtures("video"))]
async fn test_find_with_original(pool: PgPool) {
    let id = Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let video = find_with_original(&pool, &id).await.unwrap();

    assert_eq!(video.video.id, id);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_find_all_with_original_no_filter(pool: PgPool) {
    let mut pagination: Pagination<Video> = Default::default();
    pagination.limit = Some(20);
    let video_filter: Filter<Video> = Default::default();
    let original_filter: Filter<OriginalVideo> = Default::default();

    let result = find_all_with_original(&pool, pagination, video_filter, original_filter).await;
    let list = result.unwrap();
    let list_len = list.len();

    assert_eq!(list_len, 20)
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_find_all_with_original_video_filter(pool: PgPool) {
    let pagination: Pagination<Video> = Default::default();
    let mut video_filter: Filter<Video> = Default::default();
    let original_filter: Filter<OriginalVideo> = Default::default();

    video_filter.options.deleted_at = Some(None);

    let result = find_all_with_original(&pool, pagination, video_filter, original_filter).await;
    let list = result.unwrap();
    let list_len = list.len();

    assert_eq!(list_len, 10)
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_find_all_with_original_original_video_filter(pool: PgPool) {
    let pagination: Pagination<Video> = Default::default();
    let video_filter: Filter<Video> = Default::default();
    let mut original_filter: Filter<OriginalVideo> = Default::default();

    original_filter.options.url = Some("https://www.youtube.com/watch?v=1234567891".into());

    let result = find_all_with_original(&pool, pagination, video_filter, original_filter).await;
    let list = result.unwrap();
    let list_len = list.len();

    assert_eq!(list_len, 1)
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_find_all_with_original_both_filters(pool: PgPool) {
    let mut pagination: Pagination<Video> = Default::default();
    pagination.limit = Some(20);
    let mut video_filter: Filter<Video> = Default::default();
    let mut original_filter: Filter<OriginalVideo> = Default::default();

    video_filter.options.deleted_at = Some(None);

    original_filter.options.url = Some("https://www.youtube.com/watch?v=1234567890".into());

    let result = find_all_with_original(&pool, pagination, video_filter, original_filter).await;
    let list = result.unwrap();
    let list_len = list.len();

    assert_eq!(list_len, 10);

    for video in list {
        assert_eq!(
            video.original.url,
            "https://www.youtube.com/watch?v=1234567890"
        );

        assert!(video.video.deleted_at.is_none());
    }
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("user", "channel", "original_video")
)]
async fn test_create_many(pool: PgPool) {
    let mut dtos = vec![];
    let channel_ids = vec![666];
    for _ in 0..10 {
        let id = uuid::Uuid::new_v4();

        let dto = CreateVideoDto {
            id,
            title: "Test",
            description: "Test",
            user_id: 666,
            channel_ids: &channel_ids,
            language: "en",
            end_time: None,
            original_id: 666,
            start_time: "00:00:00",
            tags: None,
        };

        dtos.push(dto);
    }

    create_many(&pool, dtos).await.unwrap();

    let count = sqlx::query!("SELECT COUNT(*) FROM videos where original_video_id = 666")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert!(count.count.is_some());
    assert_eq!(count.count.unwrap(), 10);
}

#[sqlx::test(migrations = "../migrations", fixtures("video"))]
async fn test_find_by_user_id_with_original(pool: PgPool) {
    let user_id = 6666;
    let video_id = Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();

    let result = find_by_user_id_with_original(&pool, &video_id, user_id)
        .await
        .unwrap();

    assert_eq!(result.video.id, video_id);
    assert_eq!(result.video.user_id, user_id)
}

#[sqlx::test(migrations = "../migrations", fixtures("video", "user"))]
async fn test_find_by_user_id_with_original_not_found_other_user(pool: PgPool) {
    let user_id = 666;
    let video_id = Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();

    let result = find_by_user_id_with_original(&pool, &video_id, user_id).await;

    assert!(result.is_err());
}

#[sqlx::test(migrations = "../migrations", fixtures("video"))]
async fn test_find_with_original_and_video_channels(pool: PgPool) {
    let id = Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let video = find_with_original_and_video_channels(&pool, &id)
        .await
        .unwrap();

    assert_eq!(video.video.id, id);
    assert_eq!(video.original.id, 666);
    assert_eq!(video.video_channels.len(), 2);
    assert_eq!(video.video_channels[0].video_id, id);
    assert_eq!(video.video_channels[0].channel_id, 666);
    assert_eq!(video.video_channels[1].video_id, id);
    assert_eq!(video.video_channels[1].channel_id, 667);
}

#[sqlx::test(migrations = "../migrations", fixtures("video"))]
async fn test_find_with_original_and_video_channels_not_found(pool: PgPool) {
    let id = Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120095").unwrap();
    let video = find_with_original_and_video_channels(&pool, &id).await;

    assert!(video.is_err());

    let err = video.unwrap_err();

    match err {
        sqlx::Error::RowNotFound => {}
        _ => panic!("Expected RowNotFound error"),
    }
}

#[sqlx::test(migrations = "../migrations", fixtures("video"))]
async fn test_find_by_user_id_with_original_and_video_channels(pool: PgPool) {
    let user_id = 6666;
    let video_id = Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();

    let result = find_by_user_id_with_original_and_video_channels(&pool, &video_id, user_id)
        .await
        .unwrap();

    assert_eq!(result.video.id, video_id);
    assert_eq!(result.video.user_id, user_id);
    assert_eq!(result.original.id, 666);
    assert_eq!(result.video_channels.len(), 2);
    assert_eq!(result.video_channels[0].video_id, video_id);
    assert_eq!(result.video_channels[0].channel_id, 666);
    assert_eq!(result.video_channels[1].video_id, video_id);
    assert_eq!(result.video_channels[1].channel_id, 667);
}

#[sqlx::test(migrations = "../migrations", fixtures("video", "user"))]
async fn test_find_by_user_id_with_original_and_video_channels_not_found_other_user(pool: PgPool) {
    let user_id = 666;
    let video_id = Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();

    let result = find_by_user_id_with_original_and_video_channels(&pool, &video_id, user_id).await;

    assert!(result.is_err());

    let err = result.unwrap_err();

    match err {
        sqlx::Error::RowNotFound => {}
        _ => panic!("Expected RowNotFound error"),
    }
}

#[sqlx::test(migrations = "../migrations", fixtures("video"))]
async fn test_find_by_user_id_with_original_and_video_channels_not_found_video(pool: PgPool) {
    let user_id = 6666;
    let video_id = Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120095").unwrap();

    let result = find_by_user_id_with_original_and_video_channels(&pool, &video_id, user_id).await;

    assert!(result.is_err());

    let err = result.unwrap_err();

    match err {
        sqlx::Error::RowNotFound => {}
        _ => panic!("Expected RowNotFound error"),
    }
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_find_all_with_original_and_video_channels_no_filter(pool: PgPool) {
    let mut pagination: Pagination<Video> = Default::default();
    pagination.limit = Some(40);
    let video_filter: Filter<Video> = Default::default();
    let original_filter: Filter<OriginalVideo> = Default::default();

    let result =
        find_all_with_original_and_video_channels(&pool, pagination, video_filter, original_filter)
            .await;
    let list = result.unwrap();
    let list_len = list.len();

    assert_eq!(list_len, 20)
}
