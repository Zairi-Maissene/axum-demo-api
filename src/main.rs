use axum::{
    routing::{get, post},
    Router,
    extract::Json,
    Extension,
};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use tokio::net::TcpListener;
use std::env;

#[path = "./handlers/create_user.rs"] mod create_user;
#[path = "./handlers/get_users.rs"] mod get_users;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let server_address = env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let listener = TcpListener::bind(server_address).await.unwrap();
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Build our application with routes
    let app = Router::new()
        .route("/", get(get_users::get_users))
        .route("/", post(create_user::create_user))
        .with_state(pool);

    // Start the server
    axum::serve(listener,app)
        .await
        .expect("Server failed");
}

