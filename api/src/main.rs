use axum::{
    extract::{State, Path},
    http::Method,
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

// 1. Updated Structs to include release_date
#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Movie {
    id: i64,
    tmdb_id: i64,
    title: String,
    comment: String,
    user_name: String,
    poster_path: Option<String>,
    release_date: Option<String>, 
}

#[derive(Deserialize)]
struct CreateMovie {
    tmdb_id: i64,
    title: String,
    comment: String,
    user_name: String,
    poster_path: Option<String>,
    release_date: Option<String>, 
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = "sqlite://movies.db?mode=rwc";
    let pool = SqlitePoolOptions::new().connect(db_url).await?;

    // 2. Updated Table Creation
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS movies (
            id INTEGER PRIMARY KEY,
            tmdb_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            comment TEXT NOT NULL,
            user_name TEXT NOT NULL,
            poster_path TEXT,
            release_date TEXT
        )",
    )
    .execute(&pool)
    .await?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .route("/movies", get(list_movies).post(add_movie))
        .route("/movies/{id}", delete(remove_movie))
        .layer(cors)
        .with_state(pool);

    println!("🚀 Server running on http://localhost:3000");
    
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn list_movies(State(pool): State<SqlitePool>) -> Json<Vec<Movie>> {
    let movies = sqlx::query_as::<_, Movie>("SELECT * FROM movies ORDER BY id DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    Json(movies)
}

async fn add_movie(State(pool): State<SqlitePool>, Json(payload): Json<CreateMovie>) -> Json<Movie> {
    // 3. Updated Insert Query
    let id = sqlx::query("INSERT INTO movies (tmdb_id, title, comment, user_name, poster_path, release_date) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(payload.tmdb_id)
        .bind(&payload.title)
        .bind(&payload.comment)
        .bind(&payload.user_name)
        .bind(&payload.poster_path)
        .bind(&payload.release_date)
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();

    Json(Movie {
        id,
        tmdb_id: payload.tmdb_id,
        title: payload.title,
        comment: payload.comment,
        user_name: payload.user_name,
        poster_path: payload.poster_path,
        release_date: payload.release_date, 
    })
}

async fn remove_movie(State(pool): State<SqlitePool>, Path(id): Path<i64>) {
    sqlx::query("DELETE FROM movies WHERE tmdb_id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
}
