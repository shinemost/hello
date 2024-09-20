use std::thread;
fn main() {
    start_n_threads();
}
pub fn start_n_threads() {
    const N: isize = 10;

    let handles: Vec<_> = (0..N)
        .map(|i| {
            thread::spawn(move || {
                println!("Hello from a thread{}!", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
