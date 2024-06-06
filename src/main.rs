static MY_STATIC: usize = 42;
static mut MY_MUT_STATIC: usize = 42;
fn main() {
    const SECOND_HOUR: usize = 3_600;
    const SECOND_DAY: usize = 24 * SECOND_HOUR;

    println!("{SECOND_HOUR}");
    println!("{SECOND_DAY}");
    println!("{MY_STATIC}");
    unsafe {
        MY_MUT_STATIC = 32;
        println!("{MY_MUT_STATIC}");
    };
}
