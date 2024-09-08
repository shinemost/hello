fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    if a < b as i32 {
        println!("Ten is less than one hundred.");
    }


    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);

    // let a: u8 = 10;
    // let b: u16 = 1500;
    // // try_into处理转换错误，那么可以使用 TryInto
    // let b_: u8 = b.try_into().unwrap();
    // 
    // if a < b_ {
    //     println!("Ten is less than one hundred.");
    // }

    let b: i16 = 1500;

    let _b: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
}






