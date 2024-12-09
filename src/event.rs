use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TodoEvent {
    TaskAdded { task_id: String, description: String },
    TaskCompleted { task_id: String },
    TaskDeleted { task_id: String },
}

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
