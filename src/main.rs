use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    // 这个相比于层层手动往上抛异常，也太方便了
    // 想想java的throw
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    // 错误传播
    let r = read_username_from_file();
    println!("{:?}", r);
}
