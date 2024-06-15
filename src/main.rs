fn main() {
    let s1 = "hello world";
    let s2 = "hello";

    println!("no need {}", no_need(s1));
    println!("longest {}", longest2(s1, s2));


    let result: &str;
    {
        let r2 = "world";
        result = longest3(s1, r2);
        println!("longest str: {}", result);
    }
}

// 只有一个引用入参和返回值，不需要标注生命周期
fn no_need(s1: &str) -> &str {
    s1
}

// 两个入参，必须要标注生命周期，此处是将所有的入参和返回值标注为同一个生命周期
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    s1
}

// 使用了static生命周期，存在于整个程序运行期间，不建议用
fn longest2<'a>(s1: &'static str, s2: &'a str) -> &'static str {
    s1
}

// 显示声明了不同的生命周期，同时标注出生命周期之间的包含关系，比较灵活，但是不够简便。
fn longest3<'a, 'b, 'out>(s1: &'a str, s2: &'b str) -> &'out str
// 显示声明'a，'b生命周期都包含'out
    where 'a: 'out, 'b: 'out {
    if s1.len() > s2.len() {
        s1
    } else { s2 }
}


