/*
 * @LastEditors: shinemost supertain147@163.com
 * @Date: 2024-06-05 22:36:45
 * @LastEditTime: 2024-06-09 12:17:34
 * @FilePath: /hello/src/main.rs
 */
fn main() {
    let c1 = Counter::new(1);
    println!("{}", c1.number);

    c1.get_number();
    println!("{}", c1.number);
    println!("{}", c1.number);

    let mut c1 = Counter::new(2);
    c1.add(32);
    println!("mut c1 {}", c1.number);
    println!("mut c1 {}", c1.number);

    // Counter::giveup(c1);
    c1.giveup();
    // println!("mut c1 {}", c1.number);

    let c1 = Counter::new(3);
    let c2 = Counter::new(4);
    let c3 = Counter::combine(c1, c2);
    println!("{}", c3.number);
    // println!("{}", c1.number);
    // println!("{}", c2.number);
}

struct Counter {
    number: i32,
}

impl Counter {
    fn new(number: i32) -> Self {
        Counter { number }
    }

    fn get_number(&self) -> i32 {
        self.number
    }
    fn add(&mut self, increment: i32) {
        self.number += increment
    }

    fn giveup(self) {
        println!("free {}", self.number)
    }
    fn combine(c1: Self, c2: Self) -> Self {
        Counter {
            number: c1.number + c2.number,
        }
    }
}
