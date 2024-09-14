struct Foo;

impl Foo {
    fn foo(&self) {
        println!("Foo");
    }
}
fn main() {
    // Deref 可以隐式循环解引用
    let f = &&Foo;

    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&&f).foo();
}
