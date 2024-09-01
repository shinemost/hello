fn main() {
    // 格式化输出三大金刚：
    // print!
    // println!
    // format!
    println!("Hello");                 // => "Hello"
    println!("Hello, {}!", "world");   // => "Hello, world!"
    println!("The number is {}", 1);   // => "The number is 1"
    println!("{:?}", (3, 4));          // => "(3, 4)"
    println!("{value}", value = 4);      // => "4"
    println!("{} {}", 1, 2);           // => "1 2"
    println!("{:04}", 42);             // => "0042" with leading zeros

    print!("不换行");
    print!("就是任性");

    println!("=======");

    let string = format!("不成功,{}", "就成仁");
    println!("{}", string);

    //  两大护法 输出到标准错误输出 2
    // 应该仅用于输出错误信息和进度信息，其他场景都应使用println!
    eprint!("{}", "出错了");
    eprintln!("{}", "world都不行了");
}
