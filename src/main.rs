use std::sync::{Mutex, OnceLock};
//  在静态上下文中，调用被限制为常量函数、元组结构体和元组变体。
// static NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
use lazy_static::lazy_static;
use std::collections::HashMap;
// lazy_static是社区提供的非常强大的宏，用于懒初始化静态变量，之前的静态变量都是在编译期初始化的，
// 因此无法使用函数调用进行赋值，而lazy_static允许我们在运行期初始化静态变量！
lazy_static! {
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

// 现在可以直接使用标准库的std::sync::OnceLock达成相同的结果
fn hashmap() -> &'static HashMap<u32, &'static str> {
    static HASHMAP: OnceLock<HashMap<u32, &str>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    })
}

fn mutex() -> &'static Mutex<String> {
    static NAMES: OnceLock<Mutex<String>> = OnceLock::new();
    NAMES.get_or_init(|| {
        let m = Mutex::new(String::from("Sunface, Jack, Allen"));
        m
    })
}

fn main() {
    let v = NAMES.lock().unwrap();
    println!("{}", v);

    let mut v = mutex().lock().unwrap();
    v.push_str(", Myth");
    println!("{}", v);

    // 首次访问`HASHMAP`的同时对其进行初始化
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    // 后续的访问仅仅获取值，再不会进行任何初始化操作
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());

    // 首次访问`HASHMAP`的同时对其进行初始化
    println!("The entry for `0` is \"{}\".", hashmap().get(&0).unwrap());

    // 后续的访问仅仅获取值，再不会进行任何初始化操作
    println!("The entry for `1` is \"{}\".", hashmap().get(&1).unwrap());
}
