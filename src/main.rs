fn main() {
    let a1 = 0xFF;
    let a2: u32 = 0o13;
    let a3: i64 = 0b10;
    let a4: usize = 22222;

    println!("{a1},{a2},{a3},{a4}");

    println!("i8 max:{}", i8::MAX);
    println!("i8 min:{}", i8::MIN);
    println!("i16 max:{}", i16::MAX);
    println!("i16 min:{}", i16::MIN);
    println!("i32 max:{}", i32::MAX);
    println!("i32 min:{}", i32::MIN);
    println!("i64 max:{}", i64::MAX);
    println!("i64 min:{}", i64::MIN);
    println!("i128 max:{}", i128::MAX);
    println!("i128 min:{}", i128::MIN);
    println!("u64 max:{}", u64::MAX);
    println!("u64 min:{}", u64::MIN);
    println!("usize max:{}", usize::MAX);
    println!("usize min:{}", usize::MIN);
    println!("isize max:{}", isize::MAX);
    println!("isize min:{}", isize::MIN);

    println!("i8 size:{}", std::mem::size_of::<i8>());
    println!("i16 size:{}", std::mem::size_of::<i16>());
    println!("i32 size:{}", std::mem::size_of::<i32>());
    println!("i64 size:{}", std::mem::size_of::<i64>());
    println!("i128 size:{}", std::mem::size_of::<i128>());
    println!("u8 size:{}", std::mem::size_of::<u8>());
    println!("u16 size:{}", std::mem::size_of::<u16>());
    println!("u32 size:{}", std::mem::size_of::<u32>());
    println!("u64 size:{}", std::mem::size_of::<u64>());
    println!("u128 size:{}", std::mem::size_of::<u128>());
    println!("usize size:{}", std::mem::size_of::<usize>());
    println!("isize size:{}", std::mem::size_of::<isize>());
    println!("bool size:{}", std::mem::size_of::<bool>());
    println!("char size:{}", std::mem::size_of::<char>());
    println!("() size:{}", std::mem::size_of::<()>());

    let is_ok = true;
    let can_ok = false;

    println!(
        "is_ok or can_ok is {},is_ok and can_ok is {}",
        is_ok || can_ok,
        is_ok && can_ok
    );

    let s = 'a';
    let emoi = '🤣';
    println!("{s}");
    println!("{emoi}");

    println!("char as usize:{}", s as usize);
    println!("char as i64:{}", emoi as i64);
}
