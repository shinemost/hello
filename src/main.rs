use std::thread;

pub fn panic_example() {
    println!("Hello, world!");
    let h = thread::spawn(|| {
        thread::sleep(std::time::Duration::from_millis(1000));
        panic!("boom");
    });
    let r = h.join();
    //出现panic时会发生线程执行栈回退，运行结构器以及释放资源的操作。
    // 如果panic没有捕获，那么线程就会退出
    // 通过JoinHandle检查
    match r {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e),
    }
    println!("Exiting main!")
}

pub fn panic_caught_example() {
    println!("Hello, world!");
    let h = thread::spawn(|| {
        thread::sleep(std::time::Duration::from_millis(1000));
        // 如果被捕获，外部的 handle 是检查不到这个 panic 的：
        let result = std::panic::catch_unwind(|| {
            panic!("boom");
        });
        println!("panic caught, result = {}", result.is_err()); // true
    });
    let r = h.join();
    match r {
        Ok(r) => println!("All is well! {:?}", r), // here
        Err(e) => println!("Got an error! {:?}", e),
    }
    println!("Exiting main!")
}

fn main() {
    panic_caught_example();
}
