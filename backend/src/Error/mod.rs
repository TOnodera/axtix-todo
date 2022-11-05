use std::{error, fmt::Display};
#[derive(Debug)]
pub struct ServerError {
    pub message: String,
}
impl Display for ServerError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Server Error: {}", self.message)
    }
}
impl error::Error for ServerError {}

#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
}
impl Display for ValidationError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Validation Error: {}", self.message)
    }
}
impl error::Error for ValidationError {}

#[derive(Debug)]
pub struct DatabaseError {
    pub message: String,
}
impl Display for DatabaseError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Database Error: {}", self.message)
    }
}
impl error::Error for DatabaseError {}
