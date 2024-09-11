use std::str::FromStr;
fn main() {
    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    for number in text.split_whitespace().filter_map(|x| f64::from_str(x).ok()) {
        println!("{:4.2} ", number);
    }
    //     等价于下面代码
    for number in text.split_whitespace()
        .map(|x1| f64::from_str(x1))
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap()) {
        println!("{:4.2} ", number);
    }
}