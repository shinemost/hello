fn main() {
    rayon_thread_pool();
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    let (a, b) = rayon::join(|| fibonacci(n - 1), || fibonacci(n - 2));
    a + b
}

pub fn rayon_thread_pool() {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    let n = pool.install(|| fibonacci(2));
    println!("{}", n);
}

