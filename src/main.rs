fn main() {
    // 数组实现了IntoIterator特征，所以可以直接迭代
    let arr = [1, 2, 3];
    for v in arr {
        println!("{}", v);
    }

    // IntoIterator特征有一个into_inter方法，可以显式的将数组转为迭代器
    // 该方法会交出所有权, 调用此方法后无法再使用原变量,除非实现了copy特征
    // 但是数组会特别，需要查看文档。
    for m in arr.into_iter() {
        println!("{}", m);
    }
    println!("{:?}", arr);

    // 直接对数值序列进行迭代，也是很常见的使用方式。
    for ss in 0..10 {
        println!("{}", ss);
    }

    let s = vec![4, 5, 6];
    for ss in s {
        println!("{ss}");
    }

    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();

    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);

    let values = vec![10, 20, 30];
    // 使用迭代器模拟for循环
    // for循环其实就是迭代器的语法糖
    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                match iter.next() {
                    Some(x) => { println!("{}", x); }
                    None => break,
                }
            },
        };
        result
    }

    //     Iterator本身也实现了IntoIterator特征
    // Vec没有实现copy特征，所有权被移除，这里报错
    // values.into_iter();
    for v in arr.into_iter().into_iter().into_iter().into_iter().into_iter() {
        println!("{}", v)
    }

    //     转换迭代器的三个方法：
    //     into_iterator 会夺取所有权
    //     iter 不可变引用
    //     iter_mut 可变引用
    let values = vec![1, 2, 3];
    let _values_iter = values.iter();
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    let mut_values_iter = values.iter_mut();
    mut_values_iter.for_each(|x| *x *= 2);
    println!("{:?}", values);

    //     Iterator特征和IntoIterator特征的区别
    //     实现了Iterator特征的类型都可以称为迭代器，才能调用next
    //     IntoIterator 强调的是某一个类型如果实现了该特征，它可以通过 into_iter，iter 等方法变成一个迭代器

}

