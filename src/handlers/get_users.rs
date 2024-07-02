use axum::{ http::StatusCode, response::IntoResponse,  extract::State, Json };
use sqlx::PgPool;
#[path = "../models/user.rs"] mod user;

pub async fn get_users(State(db_pool): State<PgPool>)->  impl IntoResponse {
  let rows = sqlx::query_as!(user::User, "SELECT * FROM users")
    .fetch_all(&db_pool)
    .await;

   match rows {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}
