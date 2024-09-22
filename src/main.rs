use std::thread;
use std::time::Duration;

pub fn thread_park2() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        thread::park();
        // 后面的park会阻塞并且不能被释放
        thread::park();
        thread::park();
        println!("Hello from a park thread in case of unpark first!");
    });
    // 如果预先调用一股脑的 unpark 多次，然后再一股脑的调用 park 行不行
    // 答案是不行。因为一个线程只有一个令牌，这个令牌或者存在或者只有一个，
    // 多次调用 unpark 也是针对一个令牌进行的的操作，上面的代码会导致新建的那个线程一直处于 parked 状态。
    handle.thread().unpark();
    handle.thread().unpark();
    handle.thread().unpark();
    handle.join().unwrap();
}
fn main() {
    thread_park2();
}