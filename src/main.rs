// fn first(arr: &[i32]) -> Option<&i32> {
// 会报错，因为如果能取到值是直接返回&i32而不是Option
// arr.get(6)?
// }

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
