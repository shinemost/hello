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
}