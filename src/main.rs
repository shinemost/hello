// fn main() {
//     let mut s = String::new();

//     // 闭包实现了FnMut 因为对s进行了可变引用
//     let update_string = |str| s.push_str(str);

//     exec(update_string);

//     println!("{:?}", s);
// }

// // 而函数定义确是Fn 不可变引用，不匹配报错
// fn exec<'a, F: Fn(&'a str)>(mut f: F) {
//     f("hello")
// }

fn main() {
  let s = "hello ".to_string();

  // 闭包只是对s进行了不可变引用
  let update_string = |str| println!("{},{}", s, str);

  exec(update_string);

  println!("{:?}", s);
}

// 而函数定义确是Fn 不可变引用，不匹配报错
fn exec<'a, F: Fn(String)>(f: F) {
  f("world".to_string())
}
