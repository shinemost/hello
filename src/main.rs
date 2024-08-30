use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    // 错误传播
    let r = read_username_from_file().expect("出现异常了，但是我不告诉你真正原因");
    println!("{:?}", r);
}
