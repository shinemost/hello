/*
 * @LastEditors: shinemost supertain147@163.com
 * @Date: 2024-06-05 22:36:45
 * @LastEditTime: 2024-06-09 13:21:55
 * @FilePath: /hello/src/main.rs
 */
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Box::new(Point { x: 1, y: 2 });
    println!("point x: {} ,y: {}", point.x, point.y);
    let mut point = Box::new(Point { x: 1, y: 2 });
    point.x += 2;
    println!("point x: {} ,y: {}", point.x, point.y);

    let mut x = Box::new(32);
    println!("{x}");
    println!("{}", *x);
    *x += 10;
    println!("{x}");
}
