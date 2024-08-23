fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    assert_eq!(v, [1]);

    let mut m = vec![1, 2, 3];
    m.push(4);
    assert_eq!(m, [1, 2, 3, 4]);
}
