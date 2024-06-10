fn main() {
    // loop {
    //     println!("ctrl c");
    //     std::thread::sleep(time::Duration::from_secs(1));
    // }

    let mut i = 0;
    while i < 10 {
        println!("{i}");
        i += 1;
    }
    let v = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for elem in v {
        println!("{elem}");
    }

    for i in 0..10 {
        println!("{i}");
    }

    for i in 0..=10 {
        println!("{i}");
    }

    for i in 0..12 {
        if i == 10 {
            break;
        }
        println!("{i}");
    }

    for i in 0..12 {
        if i == 10 {
            continue;
        }
        println!("{i}");
    }

    // loop {
    //     println!("outer");
    //     loop {
    //         println!("inner");
    //         break;
    //     }
    // }
    's: loop {
        println!("outer");
        loop {
            println!("inner");
            break 's;
        }
    }
    // 循环的使用
    let v = [1, 2, 3, 4, 5];
    let mut s = Vec::new();
    for &n in v.iter() {
        let m = &n * &n;
        s.push(m);
    }
    println!("{:?}", s);

    // 迭代的使用
    let mut v = [1, 2, 3, 4, 5].to_vec();
    let s: Vec<i32> = v.iter().map(|&s| s * s).collect();
    println!("{:?}", s);
}
