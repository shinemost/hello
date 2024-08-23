use std::collections::HashMap;

fn main() {
    // HashMap
    let mut map = HashMap::new();
    map.insert("first", 1);
    map.insert("second", 2);
    map.insert("third", 3);
    for (k, v) in &map {
        println!("{k},{v}");
    }
    // 预先分配空间，减少内存扩容
    let map: HashMap<&str, i32> = HashMap::with_capacity(10);
    println!("{}", map.capacity());

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    // 手动遍历，手动塞值
    let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, team.1);
    }

    println!("{:?}", teams_map);

    // 使用迭代器
    let teams_map2: HashMap<_, _> = teams_list.into_iter().collect();
    println!("{:?}", teams_map2);
}
