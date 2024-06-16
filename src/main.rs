/*
 * @LastEditors: shinemost supertain147@163.com
 * @Date: 2024-06-10 15:17:39
 * @LastEditTime: 2024-06-16 23:17:39
 * @FilePath: /hello/src/main.rs
 */

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
// 可以指定多个泛型
#[derive(Debug)]
struct P<T, E> {
    x: T,
    y: E,
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: "x", y: "y" };
    println!("{:?}", p);
    println!("{:?}", p2);

    let p3 = P {
        x: 1.02,
        y: "hello",
    };
    println!("{:?}", p3);
}
