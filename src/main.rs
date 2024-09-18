#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut CONFIG: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    // Some(&mut Config {
    //     a: "A".to_string(),
    //     b: "B".to_string(),
    // })

    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    Some(Box::leak(c))
}

fn main() {
    // unsafe {
    //     // 试图将局部变量Config赋值给全局变量CONFIG
    //     CONFIG = Some(&mut Config {
    //         a: "A".to_string(),
    //         b: "B".to_string(),
    //     });
    //     // Config被回收

    //     println!("{:?}", CONFIG)
    // }

    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    unsafe {
        // 将`c`从内存中泄漏，变成`'static`生命周期，和程序一样久，就可以赋值给全局静态变量了
        CONFIG = Some(Box::leak(c));
        println!("{:?}", CONFIG);
    }

    unsafe {
        CONFIG = init();

        println!("{:?}", CONFIG)
    }
}
