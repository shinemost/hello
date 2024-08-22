trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Button with width: {}, height: {}, label: {}",
            self.width, self.height, self.label
        );
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "SelectBox with width: {}, height: {}",
            self.width, self.height
        );
        for option in &self.options {
            println!("Option: {}", option);
        }
    }
}

pub struct Screen {
    // 特征对象涉及到动态分发，与泛型的静态分发不同
    // 泛型Rust在编译期间会生成对应的类型编码，而且知道对象的大小
    // 而特征对象在编译期是不知道的，只有在程序运行时Rust才能通过引用指针知道实际调用哪个类型的方法
    // 因此会产生动态分发，实际上会影响性能，如果不涉及特征的多个不同实现类型，可直接使用泛型
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
