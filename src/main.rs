use std::fs::File;

fn main() {
    //类似unwarp但是可以打印预留的错误信息
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
