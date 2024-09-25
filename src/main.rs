use crossbeam;
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

// crossbeam 也提供了创建了 scoped thread 的功能，和标准库的 scope 功能类似，但
// 是它创建的 scoped thread 可以继续创建 scoped thread:
pub fn crossbeam_scope() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;
    crossbeam::scope(|s| {
        // 使用传递的s可以继续创建孙线程
        s.spawn(|_| {
            println!("hello from the first crossbeam scoped thread");
            dbg!(&a);
        });
        s.spawn(|_| {
            println!("hello from the second crossbeam scoped thread");
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    })
    .unwrap();
    a.push(4);
    assert_eq!(x, a.len())
}

// 使用三方库rayon实现同样的逻辑
// 这个库比上面的crossbeam功能多些，使用的人也多
// 还提供了 fifo 的 scope thread
pub fn rayon_scope() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;
    rayon::scope(|s| {
        // 使用传递的s可以继续创建孙线程
        s.spawn(|_| {
            println!("hello from the first rayon scoped thread");
            dbg!(&a);
        });
        s.spawn(|_| {
            println!("hello from the second rayon scoped thread");
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });
    a.push(4);
    assert_eq!(x, a.len())
}

fn main() {
    rayon_scope();
}
