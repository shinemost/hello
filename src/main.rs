use crossbeam;
use go_spawn::{go, go_ref, join, join_all};
use send_wrapper::SendWrapper;
use std::{
    ops::Deref,
    rc::Rc,
    sync::{
        atomic::{AtomicI64, Ordering},
        mpsc::channel,
        Arc,
    },
    thread,
};

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

pub fn send_wrapper() {
    let wrapped_value = SendWrapper::new(Rc::new(42));

    let (sender, receiver) = channel();

    let _t = thread::spawn(move || {
        sender.send(wrapped_value).unwrap();
    });

    let wrapped_value = receiver.recv().unwrap();

    let value = wrapped_value.deref();
    println!("received from the main thread: {}", value);
}

// 使用三方库go-spawn模拟golang的go开启协程语法
pub fn go_thread() {
    let counter = Arc::new(AtomicI64::new(0));

    let counter_cloned = counter.clone();

    // Spawn a thread that captures values by move.
    go! {
     for _ in 0..100 {
       counter_cloned.fetch_add(1, Ordering::SeqCst);
     }

    }
    assert!(join!().is_ok());
    assert_eq!(counter.load(Ordering::SeqCst), 1200);
}

pub fn go_thread_2() {
    static COUNTER: AtomicI64 = AtomicI64::new(0);
    for _ in 0..10 {
        // Spawn a thread that captures by reference.
        go_ref!(COUNTER.fetch_add(1, Ordering::SeqCst));
    }
    join_all!();
    assert_eq!(COUNTER.load(Ordering::SeqCst), 10);
}

fn main() {
    go_thread_2();
}
