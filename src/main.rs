use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    // 类似于java里的CyclicBarrier
    // 以及golang三方库里的CyclicBarrier
    // https://github.com/marusama/cyclicbarrier.git
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
