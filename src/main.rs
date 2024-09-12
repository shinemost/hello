fn main() {
    // 求和
    let s: u64 = (1..=100).sum();
    println!("{s}");

    // 个数 返回的是usize
    let s: usize = (1..=100).count();
    println!("{s}");

    // 乘积
    let s: u64 = (1..=20).product();
    println!("{s}");
}
