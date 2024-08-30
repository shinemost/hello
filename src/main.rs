fn first(arr: &[i32]) -> Option<&i32> {
    // ?也能适用于Option,提前返回None
    let v = arr.get(6)?;
    Some(v)
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    let r = first(&a);
    println!("{:?}", r);
}
