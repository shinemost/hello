use std::cell::{Cell, RefCell};
fn main() {
    // 要求value 必须实现copy特征
    let c = Cell::new("asdf");
    // let c = Cell::new(String::from("Hello"));
    let one = c.get();
    c.set("qwer");
    let two = c.get();
    println!("{},{}", one, two);

    // Refcell没有本质改变所有权规则，只是将校验延迟了，能够通过编译，但是运行时会报Panic
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();
    println!("{},{}", s1, s2);

    // 与 Cell 用于可 Copy 的值不同，RefCell 用于引用
    // RefCell 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
    // RefCell 适用于编译期误报或者一个引用被在多处代码使用、修改以至于难于管理借用关系时
    // 使用 RefCell 时，违背借用规则会导致运行期的 panic
}