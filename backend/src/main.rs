use std::env;

use axum::{
    extract::{State, Json},
    routing::post, Router, debug_handler,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Clone)]
struct AppState {
    pub pool: Pool<Sqlite>,
}

#[derive(Serialize, Deserialize)]
struct Prompt {
    pub content: String,
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    let state = AppState { pool };
    
    let api = Router::new()
        .route("/prompt/add", post(prompt_add));

    let app = Router::new()
        .nest("/api", api)
        .with_state(state);

    println!("listening on port 3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn prompt_add(State(state): State<AppState>, Json(json): Json<Prompt>) -> Json<Prompt> {
    sqlx::query!(r#"INSERT INTO prompts VALUES ( ?1 )"#, json.content)
        .execute(&state.pool)
        .await
        .unwrap();

    return Json(json);
}
