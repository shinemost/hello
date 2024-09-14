fn main() {
    // ----- Box智能指针，将对象分配到堆上----
    let a = Box::new(3);
    // println宏隐式地调用了 Deref 对智能指针 a 进行了解引用
    println!("a = {}", a); // a = 3

    // 下面一行代码将报错
    // 在表达式中，我们无法自动隐式地执行 Deref 解引用操作，你需要使用 * 操作符 let b = *a + 1，来显式的进行解引用
    // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`

    // --------避免栈上数据拷贝--------
    // 在栈上创建一个长度为1000的数组
    let arr = [0; 1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0; 1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());

    // -----------将动态大小类型变为 Sized 固定大小类型--------------
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    // -----------特征对象----------------------
    trait Draw {
        fn draw(&self);
    }

    struct Button {
        id: u32,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("这是屏幕上第{}号按钮", self.id)
        }
    }

    struct Select {
        id: u32,
    }

    impl Draw for Select {
        fn draw(&self) {
            println!("这个选择框贼难用{}", self.id)
        }
    }

    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];

    for e in elems {
        e.draw()
    }

    let arr = vec![Box::new(1), Box::new(2)];
    // 使用 & 借用数组中的元素，否则会报所有权错误
    let (first, second) = (&arr[0], &arr[1]);
    // 表达式不能隐式的解引用，因此必须使用 ** 做两次解引用，第一次将 &Box<i32> 类型转成 Box<i32>，第二次将 Box<i32> 转成 i32
    let _sum = **first + **second;

    let s = gen_static_str();
    println!("{}", s);
}

// 你需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久，那么就可以使用 Box::leak，
// 例如有一个存储配置的结构体实例，它是在运行期动态插入内容，那么就可以将其转为全局有效，虽然 Rc/Arc 也可以实现此功能，但是 Box::leak 是性能最高的。
fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
}
