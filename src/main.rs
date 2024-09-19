use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    // 下面代码会报错borrow of moved value: `v`
    // v的所有权已经被子线程拿走了
    // println!("{:?}", v);
}
