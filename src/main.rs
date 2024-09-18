use std::sync::atomic::{AtomicUsize, Ordering};

// 定义常量
// 关键字是const而不是let
// 定义常量必须指明类型（如 i32）不能省略
// 定义常量时变量的命名规则一般是全部大写
// 常量可以在任意作用域进行定义，其生命周期贯穿整个程序的生命周期。编译时编译器会尽可能将其内联到代码中，所以在不同地方对同一常量的引用并不能保证引用到相同的内存地址
// 常量的赋值只能是常量表达式/数学表达式，也就是说必须是在编译期就能计算出的值，如果需要在运行时才能得出结果的值比如函数，则不能赋值给常量表达式
// 对于变量出现重复的定义(绑定)会发生变量遮盖，后面定义的变量会遮住前面定义的变量，常量则不允许出现重复的定义
const MAX_ID: usize = usize::MAX / 2;
// 静态变量
// 静态变量不会被内联，在整个程序中，静态变量只有一个实例，所有的引用都会指向同一个地址
// 存储在静态变量中的值必须要实现 Sync trait
static mut REQUEST_RECV: usize = 0;

static REQUEST_RECV_ATOM: AtomicUsize = AtomicUsize::new(0);

struct Factory {
    factory_id: usize,
}

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn generate_id() -> usize {
    // 检查两次溢出，否则直接加一可能导致溢出
    let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_val > MAX_ID {
        panic!("Factory ids overflowed");
    }
    GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let next_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("Factory ids overflowed");
    }
    next_id
}

impl Factory {
    fn new() -> Self {
        Self {
            factory_id: generate_id(),
        }
    }
}

fn main() {
    println!("用户ID允许的最大值是{}", MAX_ID);
    // Rust 要求必须使用unsafe语句块才能访问和修改static变量，因为这种使用方式往往并不安全，其实编译器是对的，当在多线程中同时去修改时，会不可避免的遇到脏数据。
    unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
    };

    for _ in 0..100 {
        REQUEST_RECV_ATOM.fetch_add(1, Ordering::Relaxed);
    }

    println!("当前用户请求数{:?}", REQUEST_RECV_ATOM);

    let facotroy1 = Factory::new();
    let facotroy2 = Factory::new();
    println!("{}", facotroy1.factory_id);
    println!("{}", facotroy2.factory_id);
}
