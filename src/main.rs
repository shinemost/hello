fn main() {
  let mut s = String::new();

  // 想要在闭包内部捕获可变借用，需要把该闭包声明为可变类型
  // let mut update_string = |str| s.push_str(str);

  // 只要闭包捕获的类型都实现了Copy特征的话，这个闭包就会默认实现Copy特征
  let update_string = |str| s.push_str(str);
  exec(update_string);

  // update_string("hello");

  println!("{:?}", s);
}

// 声明闭包实现了FnMut特征，可变类型闭包
fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
  f("hello")
}