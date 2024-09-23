use std::thread;
use thread_priority::{set_current_thread_priority, ThreadBuilder, ThreadBuilderExt, ThreadExt, ThreadPriority};

pub fn start_thread_with_priority() {
    let handle1 = thread::spawn(|| {
        assert!(set_current_thread_priority(ThreadPriority::Min).is_ok());
        println!("Hello from a thread5!");
    });
    let handle2 = thread::spawn(|| {
        assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());
        println!("Hello from a thread6!");
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}

// 使用ThreadBuilderExt扩展标准库的线程创建带上优先级选项
pub fn thread_builder_std() {
    let thread = thread::Builder::new()
        .name("MyNewThread".to_owned())
        .spawn_with_priority(ThreadPriority::Max, |result| {
            // This is printed out from within the spawned thread.
            println!("Set priority result: {:?}", result);
            assert!(result.is_ok());
        }).unwrap();
    assert!(thread::current().get_priority().is_ok());
    println!("This thread's native id is: {:?}", thread::current().get_native_id());
    println!("This thread's priority  is: {:?}", thread::current().get_priority().unwrap());
    thread.join().expect("TODO: panic message");
}

// 使用ThreadBuilder构建带有优先级的线程
pub fn thread_builder() {
    let thread = ThreadBuilder::default()
        .name("MyThread")
        .priority(ThreadPriority::Max)
        .spawn(|result| {
            // This is printed out from within the spawned thread.
            println!("Set priority result: {:?}", result);
            assert!(result.is_ok());
        }).unwrap();
    thread.join().expect("TODO: panic message");

    // Another example where we don't care about the priority having been set.
    let thread = ThreadBuilder::default()
        .name("MyThread")
        .priority(ThreadPriority::Max)
        .spawn_careless(|| {
            // This is printed out from within the spawned thread.
            println!("We don't care about the priority result.");
        }).unwrap();
    thread.join().expect("TODO: panic message");
}

fn main() {
    // start_thread_with_priority();
    thread_builder_std();
    // thread_builder();
}