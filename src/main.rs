/*
 * @LastEditors: shinemost supertain147@163.com
 * @Date: 2024-06-05 22:36:45
 * @LastEditTime: 2024-06-09 13:32:21
 * @FilePath: /hello/src/main.rs
 */
// 结构体需要实现Clone 与 Copy特质，基础类型默认实现了，String没有需要自己实现。
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: f64,
    // z: String,
}

fn main() {
    let x = vec![1, 2, 3];
    // let y = x; //默认执行的是move
    let y = x.clone();
    println!("{:?}", y);
    println!("{:?}", x);

    let p1 = Point { x: 1, y: 0.1 };
    let p2 = p1; //默认也是move
    println!("{:?}", p2);
    println!("{:?}", p1);
}
