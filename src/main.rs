use std::thread;
use std::time::Duration;
use thread_control::make_pair;

pub fn control_thread() {
    let (flag, control) = make_pair();
    let handle = thread::spawn(move || {
        while flag.alive() {
            thread::sleep(Duration::from_millis(100));
            println!("I'm alive!");
        }
    });
    thread::sleep(Duration::from_millis(100));
    assert_eq!(control.is_done(), false);
    control.stop(); // Also you can `control.interrupt()` it
    handle.join().unwrap();
    assert_eq!(control.is_interrupted(), false);
    assert_eq!(control.is_done(), true);
    println!("This thread is stopped")
}
fn main() {
    control_thread();
}