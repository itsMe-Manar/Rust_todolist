use serde::{Deserialize, Serialize};

/// A simple struct representing a task.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub completed: bool,
}

/// Manage a list of tasks (state management).
#[derive(Debug, Default)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}
/*
impl TaskList {
    /// Add a new task.
    pub fn add_task(&mut self, id: String, description: String) {
        self.tasks.push(Task {
            id,
            description,
            completed: false,
        });
    }

    /// Toggle a task's completion status by its ID.
    pub fn toggle_task(&mut self, id: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = !task.completed; // Toggle completed status
        }
    }

    /// Get all tasks.
    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }*/

