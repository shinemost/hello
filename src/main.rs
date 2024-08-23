fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // 数组下标返回的是元素的引用，可能会发生数组越界的报错
    let third: &i32 = &v[2];
    v.push(6);

    println!("第三个元素是 {}", third);

    // get返回的是option，不会报错，有返回Some，无则返回None
    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }
}
