use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    // ？也可以链式调用，前面返回OK，直接传入后一个函数进行使用
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    // 错误传播
    let r = read_username_from_file().expect("出现异常了，但是我不告诉你真正原因");
    println!("{:?}", r);
}
