fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    // let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    // vec.sort_unstable();
    // assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);

    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}
