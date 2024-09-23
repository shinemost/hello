use std::cell::RefCell;
use std::thread;

pub fn start_threads_with_thread_local() {
    // 定义线程全局变量
    thread_local!(static COUNTER: RefCell<u32> = RefCell::new(1));
    COUNTER.with(|c| {
        *c.borrow_mut() = 2;
    });
    let handle1 = thread::spawn(move || {
        // 此处都是全局变量的副本，只针对当前拥有的线程
        COUNTER.with(|c| {
            *c.borrow_mut() = 3;
        });
        COUNTER.with(|c| {
            println!("Hello from a thread7, c={}!", *c.borrow());
        });
    });
    let handle2 = thread::spawn(move || {
        COUNTER.with(|c| {
            *c.borrow_mut() = 4;
        });
        COUNTER.with(|c| {
            println!("Hello from a thread8, c={}!", *c.borrow());
        });
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    COUNTER.with(|c| {
        println!("Hello from main, c={}!", *c.borrow());
    });
}
fn main() {
    start_threads_with_thread_local();
}