use crate::aggregate::TaskList;
use std::{sync::Arc};
use tokio::sync::Mutex;

// Shared state type alias
pub type SharedState = Arc<Mutex<TaskList>>;
