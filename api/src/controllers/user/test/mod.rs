use std::sync::Arc;

use actix_http::Request;
use actix_web::{
    dev::ServiceResponse,
    http::header::ContentType,
    test,
    web::{self, post},
    App,
};
use marco_polo_rs_core::database::models::user::{User, UserRole};
use reqwest::StatusCode;
use sqlx::PgPool;

use chrono::NaiveDate;

use crate::{
    auth::gen_token, mail::engine::handlebars::HandleBarsEngine, models::error::AppErrorResponse,
};
use crate::{
    controllers::{test::mock::mailer::MailSenderMock, user::dtos::create::CreateUser},
    mail::Mailer,
    AppMailer,
};

use crate::controllers::user::dtos::{
    find::UserDTO,
    forgot::{ForgotPasswordDto, ResetPasswordDto},
};
use crate::utils::test::get_token;
use crate::{controllers::user::create_user, AppPool};

use super::{find_all, find_by_id, forgot_password, login, reset_password};

#[sqlx::test(migrations = "../migrations")]
async fn test_create_user_valid_email_and_password(pool: PgPool) {
    let pool = Arc::new(pool);

    let create_user_dto = CreateUser {
        name: "Test".to_string(),
        email: "test123@gmail.com".to_string(),
        password: "12345aA!".to_string(),
        role: Some(UserRole::User),
    };

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_json(&create_user_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::CREATED);

    let record = sqlx::query!(
        r#"
        SELECT COUNT(*) FROM users WHERE name = $1
        "#,
        "Test"
    )
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    assert!(record.count.is_some());

    assert_eq!(record.count.unwrap(), 1);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_create_user_invalid_email(pool: PgPool) {
    let pool = Arc::new(pool);

    let create_user_dto = CreateUser {
        name: "Test".to_string(),
        email: "Not_a_email".to_string(),
        password: "12345aA!".to_string(),
        role: Some(UserRole::User),
    };

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_json(&create_user_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_create_user_invalid_password(pool: PgPool) {
    let pool = Arc::new(pool);

    let create_user_dto = CreateUser {
        name: "Test".to_string(),
        email: "test123@gmail.com".to_string(),
        password: "123".to_string(),
        role: Some(UserRole::User),
    };

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_json(&create_user_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/user"))]
async fn test_find_by_id_get_unauthorized(pool: PgPool) {
    let pool = Arc::new(pool);

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/666")
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/user"))]
async fn test_find_by_id_get_not_found(pool: PgPool) {
    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/665")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/user"))]
async fn test_find_by_id_get_deleted(pool: PgPool) {
    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/667")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/user"))]
async fn test_find_by_id_get_ok(pool: PgPool) {
    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref());

    let date = NaiveDate::from_ymd_opt(2022, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let expected_dto: UserDTO = UserDTO {
        id: 666,
        name: "TestUser".to_string(),
        email: "teste@gmail.com".to_string(),
        role: UserRole::User,
        created_at: date,
        updated_at: date,
    };

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/666")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: UserDTO = test::read_body_json(response).await;
    assert_eq!(actual_dto, expected_dto);
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/users"))]
async fn test_find_all(pool: PgPool) {
    let pool = Arc::new(pool);
    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: Vec<UserDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 10);
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/users"))]
async fn test_find_all_unauthorized(pool: PgPool) {
    let pool = Arc::new(pool);

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/")
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/users"))]
async fn test_find_all_15(pool: PgPool) {
    let pool = Arc::new(pool);
    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/?limit=15")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: Vec<UserDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 15);
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/users"))]
async fn test_find_all_asc(pool: PgPool) {
    let pool = Arc::new(pool);
    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/?order=asc")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: Vec<UserDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 10);

    let mut base_id: i32 = 0;
    for (index, user) in actual_dto.into_iter().enumerate() {
        if index == 0 {
            base_id = user.id;
        } else {
            assert!(user.id > base_id);
            base_id = user.id;
        }
    }
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/users"))]
async fn test_find_all_desc(pool: PgPool) {
    let pool = Arc::new(pool);
    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/?order=desc")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: Vec<UserDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 10);

    let mut base_id: i32 = 0;
    for (index, user) in actual_dto.into_iter().enumerate() {
        if index == 0 {
            base_id = user.id;
        } else {
            assert!(user.id < base_id);
            base_id = user.id;
        }
    }
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/users"))]
async fn test_find_all_error(pool: PgPool) {
    let pool = Arc::new(pool);
    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/?order_by=error")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/user"))]
async fn test_forgot_password(pool: PgPool) {
    let pool = Arc::new(pool);
    let email = "teste@gmail.com";

    let test_app = innit_test_app(pool.clone()).await;

    let forgot_password_dto = ForgotPasswordDto {
        email: email.into(),
    };

    let request = test::TestRequest::post()
        .uri("/forgot-password")
        .insert_header(ContentType::json())
        .set_json(&forgot_password_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let user: User = sqlx::query_as(
        r#"
        SELECT * FROM users WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    assert!(user.forgot_token.is_some());
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/user"))]
async fn test_forgot_password_wrong_email(pool: PgPool) {
    let pool = Arc::new(pool);
    let email = "wrong@gmail.com";

    let test_app = innit_test_app(pool.clone()).await;

    let forgot_password_dto = ForgotPasswordDto {
        email: email.into(),
    };

    let request = test::TestRequest::post()
        .uri("/forgot-password")
        .insert_header(ContentType::json())
        .set_json(&forgot_password_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;

    let status = response.status().as_u16();

    let body: AppErrorResponse = test::read_body_json(response).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body.errors.len(), 1);
    assert_eq!(body.errors[0], "User not found");
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/forgotten_users")
)]
async fn test_reset_password(pool: PgPool) {
    std::env::set_var("HASH_KEY", "test");
    let pool = Arc::new(pool);
    let token = "hello world";
    let password = "12345aA!";

    let test_app = innit_test_app(pool.clone()).await;

    let reset_password_dto = ResetPasswordDto {
        token: token.into(),
        password: password.into(),
    };

    let request = test::TestRequest::put()
        .uri("/reset-password")
        .insert_header(ContentType::json())
        .set_json(&reset_password_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;

    let status = response.status().as_u16();

    assert_eq!(status, StatusCode::OK);

    let user: User = sqlx::query_as(
        r#"
        SELECT * FROM users WHERE id = $1
        "#,
    )
    .bind(6666)
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    assert!(user.forgot_token.is_none());
    assert!(bcrypt::verify(password, &user.password).unwrap());
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/forgotten_users")
)]
async fn test_reset_password_invalid_password(pool: PgPool) {
    std::env::set_var("HASH_KEY", "test");
    let pool = Arc::new(pool);
    let token = "hello world";
    let password = "1234";

    let test_app = innit_test_app(pool.clone()).await;

    let reset_password_dto = ResetPasswordDto {
        token: token.into(),
        password: password.into(),
    };

    let request = test::TestRequest::put()
        .uri("/reset-password")
        .insert_header(ContentType::json())
        .set_json(&reset_password_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;

    let status = response.status().as_u16();

    assert_eq!(status, StatusCode::BAD_REQUEST);

    let errors: AppErrorResponse = test::read_body_json(response).await;

    assert_eq!(errors.errors.len(), 1);
    assert_eq!(errors.errors[0], "password: Must contain at least eight characters, ".to_owned()+
    "including one uppercase letter, one lowercase letter, and one number. Dont use spaces., "+
     "Must Contain At Least One Special Character");

    let user: User = sqlx::query_as(
        r#"
        SELECT * FROM users WHERE id = $1
        "#,
    )
    .bind(6666)
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    assert!(user.forgot_token.is_some());
    assert!(!bcrypt::verify(password, &user.password).unwrap());
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/forgotten_users")
)]
async fn test_reset_password_expired_token(pool: PgPool) {
    std::env::set_var("HASH_KEY", "test");
    let pool = Arc::new(pool);
    let token = "hello world 3";
    let password = "12345aA!";

    let test_app = innit_test_app(pool.clone()).await;

    let reset_password_dto = ResetPasswordDto {
        token: token.into(),
        password: password.into(),
    };

    let request = test::TestRequest::put()
        .uri("/reset-password")
        .insert_header(ContentType::json())
        .set_json(&reset_password_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;

    let status = response.status().as_u16();

    assert_eq!(status, StatusCode::NOT_FOUND);

    let errors: AppErrorResponse = test::read_body_json(response).await;

    assert_eq!(errors.errors.len(), 1);
    assert_eq!(errors.errors[0], "User not found");

    let user: User = sqlx::query_as(
        r#"
        SELECT * FROM users WHERE id = $1
        "#,
    )
    .bind(8888)
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    assert!(user.forgot_token.is_some());
    assert!(!bcrypt::verify(password, &user.password).unwrap());
}

async fn innit_test_app(
    pool: Arc<PgPool>,
) -> impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
    let pool = AppPool { pool };
    let mailer = Mailer::new(HandleBarsEngine::new("./templates"), MailSenderMock);
    let mailer = AppMailer {
        mailer: Arc::new(mailer),
    };
    let web_data = web::Data::new(pool);
    let app = App::new()
        .app_data(web_data)
        .app_data(web::Data::new(mailer))
        .route(
            "/forgot-password",
            post().to(forgot_password::<HandleBarsEngine, MailSenderMock>),
        )
        .service(create_user)
        .service(login)
        .service(find_all)
        .service(find_by_id)
        .service(reset_password);

    let test_app = test::init_service(app).await;

    return test_app;
}
