use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result_ok: Result<i32, &str> = Ok(32);
    let value = result_ok.unwrap();
    println!("{value}");

    // unwrap遇到error 会直接panic
    // let result_error: Result<i32, &str> = Err("FFF");
    // let value = result_error.unwrap();
    // println!("{value}");

    let result_ok: Result<i32, &str> = Ok(32);
    let value = result_ok?;
    println!("{value}");


    let v = vec![1, 3, 5];
    match find_first_even(v) {
        None => println!("not found even"),
        Some(i) => println!("{i}"),
    }

    let mm = "234";
    match parse_number(mm) {
        Ok(i) => println!("{i}"),
        Err(err) => println!("error,{}", err)
    }

    let mm = "d";
    match parse_number(mm) {
        Ok(i) => println!("{i}"),
        Err(err) => println!("error,{}", err)
    }


    Ok(())
}

// ？如果遇到None，提前返回
fn find_first_even(v: Vec<i32>) -> Option<i32> {
    let v = v.iter().find(|&x| x % 2 == 0)?;
    Some(*v)
}

// ? 如果遇到error,提前返回
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    let m = s.parse()?;
    println!("{s}");
    Ok(m)
}



