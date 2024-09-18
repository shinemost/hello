use std::sync::{Mutex, RwLock};

// 1.63.0以上支持将Mutex::new、RwLock::new作为常量初始化定义
const MY_MUTEX: Mutex<i32> = Mutex::new(10);
const MY_RWLOCK: RwLock<i32> = RwLock::new(0);

fn main() {
    // 在这里可以安全地使用 MY_MUTEX 和 MY_RWLOCK
    let binding = MY_MUTEX;

    let mut lock = binding.lock().unwrap();
    *lock += 1;
    println!("{}", *lock);

    let binding = MY_RWLOCK;
    let read_lock = binding.read().unwrap();
    println!("Current value: {}", *read_lock);
}
