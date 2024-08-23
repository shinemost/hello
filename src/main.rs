use ahash::AHashMap;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
// 引入第三方的哈希函数
use ahash::RandomState;
use twox_hash::RandomXxHashBuilder64;
use twox_hash::XxHash64;

fn main() {
    // 指定HashMap使用第三方的哈希函数XxHash64
    // 每次hash使用固定种子
    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));

    // 使用随机种子，每次hash都会使用随机种子，增强安全性，但是会影响性能
    let mut hash: HashMap<_, _, RandomXxHashBuilder64> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));

    // 使用ahash 性能非常好，但是不保证密码学安全
    // https://github.com/tkaitchuck/ahash
    let mut map: HashMap<i32, i32, RandomState> = HashMap::default();
    map.insert(12, 34);

    // 可直接通过该构造方法创建map
    let mut map: AHashMap<i32, i32> = AHashMap::new();
    map.insert(12, 34);
    map.insert(56, 78);
}
