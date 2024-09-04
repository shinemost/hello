fn main() {
  let s = String::new();

  let update_string = || println!("{}", s);

  exec(update_string);
  exec1(update_string);
  exec2(update_string);
}

// 三种 Fn 的关系
// 实际上，一个闭包并不仅仅实现某一种 Fn 特征，规则如下：

// 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
// 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
// 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征

fn exec<F: FnOnce()>(f: F) {
  f()
}

fn exec1<F: FnMut()>(mut f: F) {
  f()
}

fn exec2<F: Fn()>(f: F) {
  f()
}
