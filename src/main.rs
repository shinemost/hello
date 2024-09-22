use std::{io, thread};

fn main() -> io::Result<()> {
    // 并发能力是一种资源，一个机器能够提供并发的能力值，这个数值一般等价于计算机拥 有的 CPU 数(逻辑的核数)，
    // 但是在虚机和容器的环境下，程序可以使用的 CPU 核数 可能受到限制。
    // 你可以通过 available_parallelism 获取当前的并发数:
    let count = thread::available_parallelism()?.get();
    println!("{count}");
    assert!(count >= 1_usize);
    Ok(())
}