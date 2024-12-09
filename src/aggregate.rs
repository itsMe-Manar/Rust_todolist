use async_trait::async_trait;
use cqrs_es::{Aggregate, DomainEvent};
use serde::{Deserialize, Serialize};
use std::error::Error;

/// The state of the To-Do list.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TodoList {
    pub tasks: Vec<(String, String, bool)>, // (Task ID, Description, Completed)
}

/// Commands represent the actions that can be performed on the aggregate.
#[derive(Debug, Serialize, Deserialize)]
pub enum TodoCommand {
    AddTask { task_id: String, description: String },
    CompleteTask { task_id: String },
    DeleteTask { task_id: String },
}

/// Events represent state changes caused by commands.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TodoEvent {
    TaskAdded { task_id: String, description: String },
    TaskCompleted { task_id: String },
    TaskDeleted { task_id: String },
}

/// Implement `DomainEvent` for `TodoEvent`.
impl DomainEvent for TodoEvent {
    fn event_type(&self) -> String {
        match self {
            TodoEvent::TaskAdded { .. } => "TaskAdded".to_string(),
            TodoEvent::TaskCompleted { .. } => "TaskCompleted".to_string(),
            TodoEvent::TaskDeleted { .. } => "TaskDeleted".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

/// Custom error type for the To-Do list.
#[derive(Debug)]
pub struct TodoError(pub String);

impl std::fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TodoError {}

/// Implement the `Aggregate` trait for the To-Do list.
#[async_trait]
impl Aggregate for TodoList {
    type Command = TodoCommand;
    type Event = TodoEvent;
    type Error = TodoError;
    type Services = ();

    fn aggregate_type() -> String {
        "TodoList".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            TodoCommand::AddTask { task_id, description } => {
                if self.tasks.iter().any(|(id, _, _)| *id == task_id) {
                    Err(TodoError("Task ID already exists".to_string()))
                } else {
                    Ok(vec![TodoEvent::TaskAdded { task_id, description }])
                }
            }
            TodoCommand::CompleteTask { task_id } => {
                if self.tasks.iter().any(|(id, _, _)| *id == task_id) {
                    Ok(vec![TodoEvent::TaskCompleted { task_id }])
                } else {
                    Err(TodoError("Task ID not found".to_string()))
                }
            }
            TodoCommand::DeleteTask { task_id } => {
                if self.tasks.iter().any(|(id, _, _)| *id == task_id) {
                    Ok(vec![TodoEvent::TaskDeleted { task_id }])
                } else {
                    Err(TodoError("Task ID not found".to_string()))
                }
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            TodoEvent::TaskAdded { task_id, description } => {
                self.tasks.push((task_id, description, false));
            }
            TodoEvent::TaskCompleted { task_id } => {
                if let Some(task) = self.tasks.iter_mut().find(|(id, _, _)| *id == task_id) {
                    task.2 = true; // Mark as completed
                }
            }
            TodoEvent::TaskDeleted { task_id } => {
                self.tasks.retain(|(id, _, _)| *id != task_id);
            }
        }
    }
}
