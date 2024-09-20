use std::{thread, time::Duration};
fn main() {
    let handler = thread::Builder::new()
        .name("named thread".into())
        .spawn(|| {
            println!("parking the thread");
            thread::park();
            println!("Thread unparked");
        })
        .unwrap();
    thread::sleep(Duration::from_secs(2));

    println!("unpack the thread");
    handler.thread().unpark();

    handler.join().unwrap();
}
