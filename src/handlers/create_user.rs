use axum::{ http::StatusCode, response::IntoResponse, extract::State,Json };
use sqlx::PgPool;
#[path = "../models/user.rs"] pub mod user;


pub async fn create_user(State(db_pool): State<PgPool>,Json(payload): Json<user::CreateUser>) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, email)
        VALUES ($1, $2)
        RETURNING id
        "#,
        payload.username,
        payload.email
    )
    .fetch_one(&db_pool)
    .await;

    match result {
        Ok(record) => {
        (StatusCode::CREATED, Json(record.id)).into_response()
        }
        Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response()

        }
    }
}