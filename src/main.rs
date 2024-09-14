struct HasDrop1;
struct HasDrop2;
impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1!");
    }
}
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2!");
    }
}
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}
// impl Drop for HasTwoDrops {
//     fn drop(&mut self) {
//         println!("Dropping HasTwoDrops!");
//     }
// }

#[derive(Debug)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}

// Drop特征，按照定义的顺序执行drop函数
// Rust为几乎所有的类型实现了Drop特征，可以不定义但是会调用默认的drop实现
fn main() {
    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    // let _foo = Foo;
    println!("Running!");

    let foo = Foo;
    // rust不允许显示的调用析构函数
    // foo.drop();
    // drop会拿走变量的所有权，下面println会报错
    drop(foo);
    // println!("Running!:{:?}", foo);
}
