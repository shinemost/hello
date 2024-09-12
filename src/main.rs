fn main() {
    let bee_parts = ["head", "thorax", "abdomen"];
    let mut iter = bee_parts.iter();

    // 实现了DoubleEndedIterator特征的迭代器都可以从两端遍历
    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    //对于实现了DoubleEndedIterator特征的迭代器调用rev函数进行反转
    let meals = ["breakfast", "lunch", "dinner"];
    let mut iter = meals.iter().rev();
    assert_eq!(iter.next(), Some(&"dinner"));
    assert_eq!(iter.next_back(), Some(&"breakfast"));
    assert_eq!(iter.next(), Some(&"lunch"));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
}
