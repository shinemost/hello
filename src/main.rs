fn main() {
    // 同时存在了可变引用和不可变引用
    let mut y = vec![1, 2, 3, 4, 5];
    // for (index, &val) in y.iter().enumerate() {
    //     if val > 4 {
    //         y.remove(index);
    //     }
    // }

    y.retain(|&x| x <= 4);
    assert_eq!(y, vec![1, 2, 3, 4]);
}
