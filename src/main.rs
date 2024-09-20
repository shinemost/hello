use std::thread;
fn main() {
    let handler = thread::Builder::new()
        .name("named thread".into())
        .spawn(|| {
            // 获取当前执行的线程
            let handle = thread::current();
            assert_eq!(handle.name(), Some("named thread"));
        })
        .unwrap();

    handler.join().unwrap();
}
