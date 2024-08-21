pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}


pub enum Test{
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

impl Summary for Test{
    fn summarize_author(&self) ->String{
        String::from("Test")
    }
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
// 使用特征作为函数参数，限制该函数只有实现该特征的类型才能调用
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 函数返回值是实现了Summary特质的类型
fn returns_summarizable() -> impl Summary {
    // Post {
    //     title: String::from("sunface"),
    //     author: String::from("sunface"),
    //     content: String::from(
    //         "m1 max太厉害了，电脑再也不会卡",
    //     )
    // }
    // Weibo{
    //     username:String::from("weibo"),
    //     content:String::from(
    //                 "m2 max太厉害了，电脑再也不会卡",
    //             )
    // } 
    // 也可以返回实现了该特质的枚举
    Test::Clubs

}


fn main() {
    let returns_summarizable = returns_summarizable();
   let summarize = returns_summarizable.summarize_author();
   println!("{summarize}");


}
