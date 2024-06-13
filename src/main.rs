use std::fmt::Formatter;

fn main() -> Result<(), Box<MyError>> {
    match func() {
        Err(error) => println!("{:?}", error),
        Ok(()) => (),
    }
    let s = func()?;
    println!("{:?}", s);
    println!("oo");
    Ok(())
}

#[derive(Debug)]
struct MyError {
    detail: String,
}


impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.detail
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError {}", self.detail)
    }
}

fn func() -> Result<(), MyError> {
    Err(MyError {
        detail: "CustomError".to_owned(),
    })
    // Ok(())
}







