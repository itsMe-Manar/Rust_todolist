use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct TodoError(pub String);

impl Display for TodoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for TodoError {}
