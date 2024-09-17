use std::cell::Cell;

fn retain_even(nums: &mut Vec<i32>) {
    // 使用Cell的两个方法方便的将&mut [T] 类型转换成 &[Cell<T>] 类型
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..])
        .as_slice_of_cells();

    let mut i = 0;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}
fn is_even(i: i32) -> bool {
    i % 2 == 0
}
fn main() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4];
    println!("nums: {:?}", nums);
    retain_even(&mut nums);
    println!("nums: {:?}", nums);

    // 由于 Rust 的 mutable 特性，一个结构体中的字段，要么全都是 immutable，要么全部是 mutable，
    // 不支持针对部分字段进行设置。比如，在一个 struct 中，可能只有个别的字段需要修改，而其他字段并不需要修改，为了一个字段而将整个 struct 变为 &mut 也是不合理的。
    // 所以，实现 内部可变性 的 Cell 和 RefCell 正是为了解决诸如这类问题存在的，通过它们可以实现 struct 部分字段可变，而不用将整个 struct 设置为 mutable。
}