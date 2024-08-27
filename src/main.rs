use std::fs::File;

fn main() {
    //使用Result的unwarp函数，如果Some有值，直接返回，如果没有值，直接panic
    let _f = File::open("hello.txt").unwrap();
}
