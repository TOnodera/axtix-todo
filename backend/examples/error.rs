use std::error;
use std::fmt;
use tokio;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct DoubleError;

#[derive(Debug, Clone)]
struct EmptyVec;

impl error::Error for DoubleError {}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "double error.")
    }
}

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "empty vec")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    first.parse::<i32>().map_err(|e| e.into()).map(|i| 2 * i)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[tokio::main]
async fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    // ↓ここが解せない😥😥😥

    tokio::spawn(async {
        println!("{:?}", numbers);
        print(double_first(numbers));
    })
    .await;

    tokio::spawn(async {
        print(double_first(empty));
    })
    .await;

    tokio::spawn(async {
        print(double_first(strings));
    })
    .await;
}
