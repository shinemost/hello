// fn main() {
//     let s = String::from("hello, world");
//     // s在这里被转移给a
//     let a = Box::new(s);
//     // 报错！此处继续尝试将 s 转移给 b
//     let b = Box::new(s);
// }

use std::rc::Rc;
fn main() {
    // Rc引用计数智能指针
    let a = Rc::new(String::from("hello, world"));
    // 创建了一个新的 Rc<String> 智能指针并赋给变量 a，该指针指向底层的字符串数据。
    let b = Rc::clone(&a);

    // Rc::strong_count返回引用计数值
    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b))
}
