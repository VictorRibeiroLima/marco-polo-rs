macro_rules! get_token {
    ($pool:expr) => {
        {
            std::env::set_var("API_JSON_WEB_TOKEN_SECRET", "test_secret");
            let user = sqlx::query_as!(
                marco_polo_rs_core::database::models::user::User,
                r#"SELECT id, 
                name, 
                email, 
                password, 
                role as "role: UserRole",
                created_at as "created_at: chrono::NaiveDateTime",
                updated_at as "updated_at: chrono::NaiveDateTime",
                deleted_at as "deleted_at: chrono::NaiveDateTime",
                forgot_token,
                forgot_token_expires_at
                FROM users WHERE id = 666"#
            )
            .fetch_one($pool)
            .await
            .unwrap();

            let token = crate::auth::gen_token(user).await.unwrap();
            token
        }
    };

    ($pool:expr,$id:expr) => {
        {
            std::env::set_var("API_JSON_WEB_TOKEN_SECRET", "test_secret");
            let user = sqlx::query_as!(
                marco_polo_rs_core::database::models::user::User,
                r#"SELECT id, 
                name, 
                email, 
                password, 
                role as "role: marco_polo_rs_core::database::models::user::UserRole",
                created_at as "created_at: chrono::NaiveDateTime",
                updated_at as "updated_at: chrono::NaiveDateTime",
                deleted_at as "deleted_at: chrono::NaiveDateTime",
                forgot_token,
                forgot_token_expires_at
                FROM users WHERE id = $1"#,
                $id
            )
            .fetch_one($pool)
            .await
            .unwrap();

            let token = crate::auth::gen_token(user).await.unwrap();
            token
        }
    };
}

pub(crate) use get_token;
