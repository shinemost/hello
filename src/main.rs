use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // String存储在堆上，没有实现Copy特征，直接会移交所有权
        // let s = String::from("我，飞走咯!");
        // 如果实现了Copy特征，会直接复制
        let s = 1;
        tx.send(s).unwrap();
        println!("val is {}", s);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
