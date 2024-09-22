use std::thread;
use std::time::Duration;

pub fn thread_park2() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        thread::park();
        println!("Hello from a park thread in case of unpark first!");
    });
    // 先执行unpark会导致park后立即返回
    handle.thread().unpark();
    handle.join().unwrap();
}

fn main() {
    thread_park2();
}