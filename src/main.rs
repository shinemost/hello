fn main() {
    let a = 1;
    let b = 2;
    let c = add(a, b);
    println!("{c}");

    let mut x = 1;
    //只是将实参的值复制给了形参，形参的改变并不会改变实参。
    change_i32(x);
    println!("{x}");

    // 可变引用传递，指向同一个内存对象，通过解引用即可实现修改。
    modify_i32(&mut x);
    println!("{x}");

    let p = Point {
        x: 1,
        y: 2,
    };
    print_point(p);
    println!("{:?}", p);//函数形参为结构体，默认会发生所有权的转移，除非该结构体实现Copy与Clone特质，即可改变默认行为。
}


#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn change_i32(mut x: i32) {
    x += 4;
}

fn modify_i32(x: &mut i32) {
    *x += 4;
}

fn print_point(point: Point) {
    println!("{:?}", point);
}
