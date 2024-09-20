use std::cell::RefCell;
use std::thread::LocalKey;

thread_local! {
    static FOO: RefCell<usize> = RefCell::new(0);
}
struct Bar {
    foo: &'static LocalKey<RefCell<usize>>,
}
impl Bar {
    fn constructor() -> Self {
        Self { foo: &FOO }
    }
}
fn main() {
    let bar = Bar::constructor();
    bar.foo.with(|x| println!("{:?}", x));
}
