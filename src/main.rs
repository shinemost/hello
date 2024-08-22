trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {} by its mom", Dog::baby_name());

    // println!(
    //     "A baby dog is called a {} by other animals",
    //     Animal::baby_name()
    // );
    // 使用函数的完全限定语法调用
    println!(
        "A baby dog is called a {} by other animals",
        <Dog as Animal>::baby_name()
    );
}
