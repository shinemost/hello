use std::fs::File;

fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    // ？可以进行隐式转换，自动将std::io::Error转换成了std::error::Error
    // 因为std::io::Error实现了std::error::Error
    let f = File::open("hello.txt")?;
    Ok(f)
}

fn main() {
    // 错误传播
    let r = open_file();
    println!("{:?}", r);
}
