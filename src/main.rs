use std::string;

fn main() {
    let tup = (12, "sdfd", 3.2);
    println!("tup elements {},{},{}", tup.0, tup.1, tup.2);

    println!("tup {:?}", tup);

    // tup.0 = 32;

    let mut tup2 = (12, "sdfd", 3.2);
    println!("tup2 {:?}", tup2);
    tup2.0 = 999;
    println!("tup2 {:?}", tup2);

    let mut arr = [1, 2, 33];
    println!(
        "arr len is {}, elemets is {},{},{}",
        arr.len(),
        arr[0],
        arr[1],
        arr[2]
    );
    arr[0] = 999;
    println!("arr first element is {}", arr[0]);

    let s = 32;
    let _ss = s;
    println!("{s}");

    //基本类型、元组、数组复制给其他变量不会发生所有全的转移，都实现了copy
    let _tup3 = tup;
    println!("{:?}", tup);

    //对于string 赋值给其他变量会造成所有权的转移 move
    let m = String::from("m");
    let n = m;
    println!("{m}");
}
