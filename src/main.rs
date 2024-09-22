use num_cpus;
use num_threads;
use std::thread;

fn main() {
    // 获取CPU逻辑个数
    let num = num_cpus::get();
    println!("cores : {num}");

    let count = thread::available_parallelism().unwrap().get();
    println!("available_parallelism: {}", count);

    if let Some(count) = num_threads::num_threads() {
        println!("num_threads: {}", count);
    } else {
        // num_threads不支持windows平台
        println!("num_threads: not supported");
        let count = thread_amount::thread_amount();
        if !count.is_none() {
            println!("thread_amount: {}", count.unwrap());
        }

        let count = num_cpus::get();
        println!("num_cpus: {}", count);
    }
}