use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub const SERVER: (&str, u16) = ("127.0.0.1", 8080);
