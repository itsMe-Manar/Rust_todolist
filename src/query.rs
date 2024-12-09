use crate::aggregate::TodoList;
use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query};
use std::sync::RwLock;
use cqrs_es::Aggregate;


pub struct LoggingQuery {
    pub current_state: RwLock<TodoList>, // Thread-safe mutable state
}

impl LoggingQuery {
    pub fn new() -> Self {
        Self {
            current_state: RwLock::new(TodoList::default()),
        }
    }
}

#[async_trait]
impl Query<TodoList> for LoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<TodoList>]) {
        println!("Processing events for Aggregate ID: {}", aggregate_id);

        // Update the current state
        {
            let mut state = self.current_state.write().unwrap(); // Acquire write lock
            for event in events {
                state.apply(event.payload.clone());
                println!(
                    "Event Sequence: {}, Applied Event: {:#?}",
                    event.sequence, event.payload
                );
            }
        }

        // Display the current state
        {
            let state = self.current_state.read().unwrap(); // Acquire read lock
            println!("\n--- Current To-Do List State ---");
            for (id, description, completed) in &state.tasks {
                println!(
                    "Task ID: {}, Description: {}, Completed: {}",
                    id, description, completed
                );
            }
            println!("--------------------------------\n");
        }
    }
}
