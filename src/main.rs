use std::fs::read_to_string;

// fn main() -> Result<(), std::io::Error> {
//     let html = render()?;
//     println!("{}", html);
//     Ok(())
// }

// fn render() -> Result<String, std::io::Error> {
//     let file = std::env::var("MARKDOWN")?;
//     let source = read_to_string(file)?;
//     Ok(source)
// }

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let html = render()?;
    println!("{}", html);
    Ok(())
}

fn render() -> Result<String, Box<dyn Error>> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}
