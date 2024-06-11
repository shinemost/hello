fn dived(x: i32, y: i32) -> Result<f64, String> {
    if y == 0 {
        Err("can not be zero".to_string())
    } else {
        let x = x as f64;
        let y = y as f64;
        Ok(x / y)
    }
}

fn find_elements(v: &[i32], target: i32) -> Option<usize> {
    for (index, element) in v.iter().enumerate() {
        if *element == target {
            return Some(index);
        }
    }
    None
}


fn main() {
    match dived(1, 2) {
        Ok(number) => println!("{number}"),
        Err(msg) => println!("{msg}"),
    };
    match dived(1, 0) {
        Ok(number) => println!("{number}"),
        Err(msg) => println!("{msg}"),
    };

    let v = vec![1, 2, 3, 4, 5];
    match find_elements(&v, 4) {
        None => println!("can not found"),
        Some(index) => println!("found in {index}"),
    };

    match find_elements(&v, 7) {
        None => println!("can not found"),
        Some(index) => println!("found in {index}"),
    };

    // v[43];//被动引发panci

    panic!("主动引发panic");

    //执行查看堆栈信息
//      RUST_BACKTRACE=1 cargo run
//      RUST_BACKTRACE=full cargo run
}




