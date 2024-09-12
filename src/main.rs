fn main() {
    // 使用fold模拟计数、求和、乘积
    let a = [5, 4, 3, 2, 1];
    assert_eq!(a.iter().fold(0, |n, _| n + 1), 5);
    assert_eq!(a.iter().fold(0, |n, i| n + i), 15);
    assert_eq!(a.iter().fold(1, |n, i| n * i), 120);
}
