use std::collections::HashMap;

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // v1_iter 是借用了 v1，因此 v1 可以照常使用
    println!("{:?}", v1);

    // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
    // println!("{:?}", v1_iter);

    let v1: Vec<i32> = vec![1, 2, 3];

    // 这里的 map 方法是一个迭代者适配器，它是惰性的，不产生任何行为，因此我们还需要一个消费者适配器进行收尾
    // v1.iter().map(|x| x + 1);
    // collect就是一个消费者适配器
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    // zip将两个迭代器压成一个迭代器，迭代器里的元素是元祖形式，第一个元素来自第一个迭代器，第二个元素来自第二个迭代器
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

    println!("{:?}", folks);
}

struct Shoe {
    size: u32,
    style: String,
}

// 使用闭包很方便，可以捕获环境值，这里的shoe_size就是
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

