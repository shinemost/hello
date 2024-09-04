use std::thread;
fn fn_once<F>(func: F)
where
    // FnOnce会转移所有权
    // F: FnOnce(usize) -> bool,
    F: FnOnce(usize) -> bool + Copy, // 改动在这里，调用时使用的将是它的拷贝，所以并没有发生所有权的转移。
{
    println!("{}", func(3));
    // 所有权已经转移，发生再次调用会报错
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
    // v 所有权已经被闭包获取了
    println!("{:?}", v);
}
