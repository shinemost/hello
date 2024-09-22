use std::thread;
use std::time::Duration;

pub fn start_thread_with_sleep() {
    let handle1 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2000));
        println!("Hello from a thread3");
    });
    let handle2 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        println!("Hello from a thread4");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

// 它至少保证当前线程 sleep 指定的时间。因为它会阻塞当前的线程，所以不要在异步 的代码中调用它。
// 如果时间设置为 0, 不同的平台处理是不一样的，Unix 类的平台会立 即返回，不会调用 nanosleep 系统调用，而 Windows 平台总是会调用底层的 Sleep 系统调用。
// 如果你只是想让渡出时间片，你不用设置时间为 0，而是调用 yield_now 函 数即可
pub fn start_thread_with_yield_now() {
    let handle1 = thread::spawn(|| {
        thread::yield_now();
        println!("yield_now");
    });
    let handle2 = thread::spawn(|| {
        thread::yield_now();
        println!("yield_now in another thread!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
fn main() {
    // start_thread_with_sleep();
    start_thread_with_yield_now();
}