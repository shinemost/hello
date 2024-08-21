// 通过 derive 派生特征，例如Debug,Clone有自动实现对应的默认特质代码，继承相应的功能
#[derive(Debug)]
pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

#[derive(Clone, Debug)]
pub struct Weibo {
    pub username: String,
    pub content: String,
}

#[derive(Debug)]
pub enum Test {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

fn main() {
    let post = Post {
        title: "蓬蓬".to_string(),
        author: "科学家".to_string(),
        content: "官网也不是专业的".to_string(),
    };
    println!("{:?}", post);
    println!("{:?}", Test::Clubs);

    let w1 = Weibo {
        username: "蓬蓬".to_string(),
        content: "官网也不是专业的".to_string(),
    };

    let w2 = w1.clone();

    println!("{:?}", w2);
}
