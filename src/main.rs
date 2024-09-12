fn main() {
    // chain链接，将两个可迭代者拼接在一起
    let v: Vec<_> = (1..4).chain([20, 30, 40]).collect();
    assert_eq!(v, [1, 2, 3, 20, 30, 40]);

    // 如果两个可迭代者是可逆向的，那么生成的迭代器也是可逆的
    let v: Vec<_> = (1..4).chain([20, 30, 40]).rev().collect();
    assert_eq!(v, [40, 30, 20, 3, 2, 1]);

    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let mut iter = a1.iter().chain(a2.iter());

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&6));
    assert_eq!(iter.next(), None);

    // 只要实现了IntoIterator特征的可迭代者都可以做链接，而不仅仅是迭代器
    let s1 = &[1, 2, 3];
    let s2 = &[4, 5, 6];

    let mut iter = s1.iter().chain(s2);

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&6));
    assert_eq!(iter.next(), None);
}
