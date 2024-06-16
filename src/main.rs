fn swap<T>(x: T, y: T) -> (T, T) {
    (y, x)
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    // 返回引用，不用返回示例，针对基本类型和字符串等分两种情况，默认实现COPY和未显示copy的，
    // 即存在所有权的区别。
    fn get_reference(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

fn main() {
    let x = 3;
    let y = 4;
    let r1 = swap(x, y);
    println!("{:?}", r1);

    let x = 3.01;
    let y = 4.01;
    let r1 = swap(x, y);
    println!("{:?}", r1);

    let ruslt = swap("hello", "world");
    println!("{:?}", ruslt);

    let i32_point = Point::new(1, 2);
    let (x, y) = i32_point.get_reference();
    println!("i32 point x: {},y: {}", x, y);

    let f64_point = Point::new(1.2, 2.1);
    let (x, y) = f64_point.get_reference();
    println!(" f64 point x: {},y: {}", x, y);

    let string_point = Point::new("hello".to_owned(), "world".to_string());
    let (x, y) = string_point.get_reference();
    println!("string point x: {},y: {}", x, y);
}
