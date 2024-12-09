mod aggregate;
mod query;

use aggregate::{TodoCommand, TodoList};
use cqrs_es::{mem_store::MemStore, CqrsFramework};
use query::LoggingQuery;

#[tokio::main]
async fn main() {
    // In-memory event store
    let event_store = MemStore::<TodoList>::default();

    // Logging query to track state changes
    let query = LoggingQuery::new();
    let cqrs = CqrsFramework::new(event_store, vec![Box::new(query)], ());

    // Define the aggregate ID
    let aggregate_id = "todo_list";

    // Execute commands
    let commands = vec![
        TodoCommand::AddTask {
            task_id: "1".to_string(),
            description: "Buy groceries".to_string(),
        },
        TodoCommand::AddTask {
            task_id: "2".to_string(),
            description: "Learn Rust".to_string(),
        },
        TodoCommand::CompleteTask {
            task_id: "1".to_string(),
        },
        TodoCommand::DeleteTask {
            task_id: "2".to_string(),
        },
    ];

    for command in commands {
        match cqrs.execute(aggregate_id, command).await {
            Ok(_) => println!("Command executed successfully"),
            Err(err) => eprintln!("Error executing command: {}", err),
        }
    }
}
