use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum TodoCommand {
    AddTask { task_id: String, description: String },
    CompleteTask { task_id: String },
    DeleteTask { task_id: String },
}
