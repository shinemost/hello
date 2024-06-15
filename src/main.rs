fn main() {
    let mut s = String::from("hello");
    // 不可变引用，可以同时有多个不可变引用
    let r1 = &s;
    let r2 = &s;

    println!("{r1},{r2}");
    let r3 = &mut s;
    // 可变引用具有排他性
    println!("{r3}");

    // println!("{r1},{r2}");

    let result: &str;
    {
        // result = "world";
        let r4 = &s;
        result = ff(r4);
    }
    println!("{}", result);
    // println!("{}", r4);//出了生命周期
}

fn ff<'a>(s: &'a str) -> &'a str {
    s
}