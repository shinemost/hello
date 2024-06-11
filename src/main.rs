fn func_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn mute(x: i32) -> i32 {
    x * x
}

fn add(x: i32) -> i32 {
    x + x
}

fn main() {
    let x = 3;
    let r = func_twice(mute, x);
    println!("{r}");

    let x = 10;
    let r = func_twice(add, x);
    println!("{r}");

    let v: Vec<i32> = [1, 2, 3, 4, 5, 6, 7].to_vec();
    let map: Vec<i32> = v.iter().map(|&x| x * 2).collect();
    println!("{:?}", map);

    let v: Vec<_> = [1, 2, 3, 4, 5, 6, 7].to_vec();
    let filter = v.iter().filter(|&x| x % 2 == 0).collect::<Vec<_>>();
    println!("{:?}", filter);

    let v: Vec<_> = [1, 2, 3, 4, 5, 6, 7].to_vec();
    let sum = v.iter().fold(0, |acc, &x| acc + x);
    println!("{sum}");
}




