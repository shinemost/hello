fn main() {
    // fold折叠 累积某种结果 给定初始值：累加器，加上闭包函数描述动作，动作结果与累加器作为下一个累加器反复调用闭包
    // 最终累加器的值就是fold返回的结果
    let a = ["Package", "my", "box", "with", "five", "dozen", "liquor", "jugs"];
    let pangram = a.iter().fold(String::new(), |s, w| s + w + " ");
    println!("{}", pangram);

    // 需要双端迭代器的支持
    let pangram = a.iter().rfold(String::new(), |s, w| s + w + " ");
    println!("{}", pangram);
}
