#![allow(unused)]
fn main() {
    use std::collections::HashMap;
    use std::hash::Hash;
    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        // 此处获取key之后，对map的可变借用就应该结束了
        // 但是由于编译器不够智能，会扩大map的可变借用的作用域范围，直到整个match语句结束

        match map.get_mut(&key) {
            Some(value) => value,
            None => {
                // 造成在此处又发生了map的可变借用，发生报错
                map.insert(key.clone(), V::default());
                map.get_mut(&key).unwrap()
            }
        }
    }
}

