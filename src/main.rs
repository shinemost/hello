fn main() {
    rayon_thread_pool2();
}

scoped_tls::scoped_thread_local!(static POOL_DATA:Vec<i32>);
pub fn rayon_thread_pool2() {
    let pool_data = vec![1, 2, 3];
    // We haven’t assigned any TLS data yet.
    assert!(!POOL_DATA.is_set());
    rayon::ThreadPoolBuilder::new().build_scoped(
        // 第一个闭包用于定义每个线程在启动时要执行的操作。它将 pool_data 的引用设置为 POOL_DATA 的线程本地存储值，
        // 并在一个新的线程中运行 thread.run()，这 个闭包的目的是为每个线程设置线程本地存储数据
        |thread| POOL_DATA.set(&pool_data, || thread.run()),
        // 第二个闭包定义了线程池启动后要执行的操作。它使用 pool.install 方法来确保在线 程池中的每个线程中都能够访问到线程本地存储的值，
        // 并且执行了一个断言 来验证 POOL_DATA 在这个线程的线程本地存储中已经被设置。
        |pool| pool.install(|| assert!(POOL_DATA.is_set())),
    ).unwrap();
    // 在线程池的作用域结束后，这一行代码用来释放 pool_data 变量。这是因为线程本地存储中的值是按线程管理的，
    // 所以在这个作用域结束后， 我们需要手动释放 pool_data，以确保它不再被任何线程访问。
    drop(pool_data);
}

