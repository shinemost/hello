fn main() {
    let s = String::from("sss");
    let _s2 = s.to_owned();
    println!("{s}");
    let m = dangle();
    let n = dangle2();
    println!("{m},{n}");

    println!("{}", first_world("hello world"));
    println!("{}", first_world("we are the world"));
}

fn dangle() -> String {
    "1111".to_owned()
}
// 静态的生命周期 整个程序内有效
fn dangle2() -> &'static str {
    "2222"
}

// string 与 &str == u8集合的引用
// 函数只有一个引用传入以及一个引用传出，即可不申明生命周期
fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
