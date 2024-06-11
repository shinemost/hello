// 返回copy
fn copy_back() -> i32 {
    4
}

// 不返回copy,直接移交所有权
fn copy_no_back() -> String {
    "hello".to_string()
}

// 返回引用
fn borrow_back(x: i32) -> &'static str {
    if x == 0 { "🤣😁" } else { "😭😎" }
}


fn main() {
    let x = copy_back();
    println!("{x}");
    let y = copy_no_back();
    println!("{y}");

    let s = borrow_back(x);
    println!("{s}");
}




