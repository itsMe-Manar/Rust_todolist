use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use serde::Deserialize;
use deadpool_postgres::Pool;

// Fetch all tasks
pub async fn get_tasks(State(db_pool): State<Pool>) -> impl IntoResponse {
    let client = db_pool.get().await.unwrap();
    let rows = client.query("SELECT id, description, completed FROM tasks", &[]).await.unwrap();

    let tasks: Vec<_> = rows.iter().map(|row| {
        let id: i32 = row.get(0);
        let description: String = row.get(1);
        let completed: bool = row.get(2);
        serde_json::json!({
            "id": id,
            "description": description,
            "completed": completed
        })
    }).collect();

    axum::response::Json(tasks)
}

// Add a task
#[derive(Deserialize)]
pub struct NewTask { // i still have to work on this because it does increment automatically but shouldnt be 
    pub description: String,
}

pub async fn add_task(State(db_pool): State<Pool>, Json(new_task): Json<NewTask>) -> impl IntoResponse {
    let client = db_pool.get().await.unwrap();
    client.execute(
        "INSERT INTO tasks (description) VALUES ($1)",
        &[&new_task.description],
    ).await.unwrap();
    "Task added successfully"
}

// Toggle task completion
#[derive(Deserialize)]
pub struct ToggleTask {
    pub id: i32,
}

pub async fn toggle_task(State(db_pool): State<Pool>, Json(toggle_task): Json<ToggleTask>) -> impl IntoResponse {
    let client = db_pool.get().await.unwrap();
    client.execute(
        "UPDATE tasks SET completed = NOT completed WHERE id = $1",
        &[&toggle_task.id],
    ).await.unwrap();
    "Task toggled successfully"
}
