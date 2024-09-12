use std::iter::{once, repeat};

fn main() {
    // 儿童文字游戏，1-100 被3整除的数替换成fizz，被5整除的数替换成buzz,被3与5都能整除的数替换成fizzbuzz
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });
    for line in fizz_buzz {
        println!("{line}");
    }
}
