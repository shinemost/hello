use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 由于Add 特征的定义是默认泛型参数是自己，pub trait Add<Rhs = Self>
// 因此如果是同类型相加则可直接不需要定义泛型类型
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

// 而此实现则是不同类型相加的逻辑，则必须定义泛型的类型
// 1.减少实现的样板代码
// 2.扩展类型但是无需大幅修改现有的代码
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(Millimeters(1) + Meters(10), Millimeters(10001));
}
