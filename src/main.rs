fn main() {
    let x = 1;
    let y = String::from("hello");
    print_i32_string(x, y);
    println!("{x}");
    // println!("{y}");//String 默认没有实现clone与copy特质，调用函数会移交所有权，等函数调用完会直接销毁。

    //不可变引用，不会移交所有权，也不会被修改值
    let y = String::from("hello");
    print_i32(&x);
    println!("{x}");
    print_string_borrow(&y);
    println!("{y}");

    // 可变借用，不会移交所有权，能被修改，通过*解引用
    let mut x = 2;
    let mut y = String::from("WORLD");
    modify_i32(&mut x);
    modify_string(&mut y);
    println!("{x}");
    println!("{y}");

    let mut p = Point {
        x: 1,
        y: 2,
    };
    println!("{:?}", p);
    modify_point(&mut p);
    println!("{:?}", p);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}


fn print_i32_string(x: i32, y: String) {
    println!("x is {x}");
    println!("y is {y}");
}

fn print_i32(x: &i32) {
    println!("{}", *x);
}

fn print_string_borrow(y: &String) {
    println!("{}", *y);
}

fn modify_i32(x: &mut i32) {
    (*x) += 4;
}

fn modify_string(y: &mut String) {
    *y = "00".to_string();
}

fn modify_point(p: &mut Point) {
    (*p).x = 5;
    (*p).y = 6;
}



