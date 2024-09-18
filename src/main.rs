// 低于Rust 1.70版本中， OnceCell 和 SyncOnceCell 的API为实验性的 ，
// 需启用特性 `#![feature(once_cell)]`。
// #![feature(once_cell)]
// use std::{lazy::SyncOnceCell, thread};

// Rust 1.70版本以上,
use std::{sync::OnceLock, thread};

fn main() {
    // 子线程中调用
    let handle = thread::spawn(|| {
        let logger = Logger::global();
        logger.log("thread message".to_string());
    });

    // 主线程调用
    let logger = Logger::global();
    logger.log("some message".to_string());

    let logger2 = Logger::global();
    logger2.log("other message".to_string());

    handle.join().unwrap();
}

#[derive(Debug)]
struct Logger;

// 低于Rust 1.70版本
// static LOGGER: SyncOnceCell<Logger> = SyncOnceCell::new();

// Rust 1.70版本以上
static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    fn global() -> &'static Logger {
        // 获取或初始化 Logger
        LOGGER.get_or_init(|| {
            println!("Logger is being created..."); // 初始化打印
            Logger
        })
    }

    fn log(&self, message: String) {
        println!("{}", message)
    }
}

// 编译期初始化的全局变量，const创建常量，static创建静态变量，Atomic创建原子类型
// 运行期初始化的全局变量，lazy_static用于懒初始化，Box::leak利用内存泄漏将一个变量的生命周期变为'static
// 使用标准库cell::OnceCell 和 sync::OnceLock 前者用于单线程，后者用于多线程，它们用来存储堆上的信息，并且具有最 多只能赋值一次的特性
