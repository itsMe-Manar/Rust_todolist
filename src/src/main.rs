mod aggregate;
mod routes;
mod state;
mod db;

use axum::{Router};
use tower_http::services::ServeDir;
use std::sync::Arc;
use db::create_pool;

#[tokio::main]
async fn main() {
    // Initialize the database connection pool
    let db_pool = create_pool().await;

    // Define routes
    let app = Router::new()
        .route("/tasks", axum::routing::get(routes::get_tasks))
        .route("/add-task", axum::routing::post(routes::add_task))
        .route("/toggle-task", axum::routing::post(routes::toggle_task))
        .nest_service("/", ServeDir::new("static")) // Serve the static folder
        .with_state(db_pool);

    // Run the server
    let addr = "127.0.0.1:8080".parse().unwrap();
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
