// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u32,
// }

// impl Person {
//     fn new(name: String, age: u32) -> Person {
//         Person { name, age }
//     }
// }

// fn main() {
//     let mut people = vec![
//         Person::new("Zoe".to_string(), 25),
//         Person::new("Al".to_string(), 60),
//         Person::new("John".to_string(), 1),
//     ];
//     // 定义一个按照年龄倒序排序的对比函数
//     people.sort_unstable_by(|a, b| b.age.cmp(&a.age));

//     println!("{:?}", people);
// }

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("Al".to_string(), 30),
        Person::new("John".to_string(), 1),
        Person::new("John".to_string(), 25),
    ];

    people.sort_unstable();

    println!("{:?}", people);
}
