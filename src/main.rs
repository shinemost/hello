fn main() {
    // 数组，且是在编译时已知长度，所以可以实现在栈上，同时也有 Copy 特征。而 Vec 则都是存储在堆上的。
    let names = vec!["sunface", "sunfei"];
    let ages = [18, 18];
    // 两个 names 的地址都一样，指向同一个引用。
    println!("{:?} {:?}", names.as_ptr(), ages.as_ptr());
    // 在这里 names 会发生 move
    let names2 = names;
    let ages2 = ages;
    // println!("{:?} {:?}", names.as_ptr(), ages.as_ptr());
    // 两个 ages 的地址不一样，为 copy
    println!("{:?} {:?}", names2.as_ptr(), ages2.as_ptr())

    //     所以才会出现在迭代器into_iterator方法调用后数组和vec的不同
}






