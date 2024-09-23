// pub fn wrong_start_threads_without_scoped() {
//     let mut a = vec![1, 2, 3];
//     let mut x = 0;
//     thread::spawn(move || {
//         println!("hello from the first scoped thread");
//         // a的所有权已经被转移了
//         dbg!(&a);
//     });
//     thread::spawn(move || {
//         println!("hello from the second scoped thread");
//         x += a[0] + a[2];
//     });
//     println!("hello from the main thread");
//     // After the scope, we can modify and access our variables again:
//     a.push(4);
//     assert_eq!(x, a.len());
// }

use std::thread;

pub fn start_scoped_threads() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;
    // 它可以借用 scope 外部的非 ‘static’ 数据。使用 thread::scope 函数提供的 Scope 的参数，可以创建 (spawn) scoped thread。
    // 创 建出来的 scoped thread 如果没有手工调用 join , 在这个函数返回前会自动 join
    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });
    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}
fn main() {
    start_scoped_threads();
}