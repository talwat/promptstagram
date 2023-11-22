use core::panic;
use std::env;

use axum::{
    debug_handler,
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use backend::{db, Prompt};
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Clone)]
struct AppState {
    pub pool: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    let state = AppState { pool };

    let api = Router::new()
        .route("/prompt/add", post(prompt_add))
        .route("/prompt/get/:id", get(prompt_get));

    let app = Router::new().nest("/api", api).with_state(state);

    println!("listening on port 3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn prompt_add(
    State(state): State<AppState>,
    Json(json): Json<Prompt>,
) -> Result<impl IntoResponse, StatusCode> {
    let Ok(id) = db::insert_prompt(json, &state.pool).await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(prompt) = db::get_prompt(id, &state.pool).await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    Ok(Json(prompt))
}

#[debug_handler]
async fn prompt_get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    let prompt = match db::get_prompt(id, &state.pool).await {
        Ok(prompt) => prompt,
        Err(err) => match err {
            sqlx::Error::RowNotFound => return Err(StatusCode::NOT_FOUND),
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    Ok(Json(prompt))
}
