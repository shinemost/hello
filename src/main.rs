fn main() {
    // inspect 探查，一般用于Debug程序看看是否按照预想的运行，或者打印错误日志
    let upper_case: String = "groβe"
        .chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!("after: {:?}", c))
        .collect();
    assert_eq!(upper_case, "GROΒE");

    // Debug程序，此处只有偶数才能进入第二个inspect
    let a = [1, 4, 2, 3];
    let sum = a
        .iter()
        .cloned()
        .inspect(|x| println!("about to filter: {x}"))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("made it through filter: {x}"))
        .fold(0, |sum, i| sum + i);

    println!("{sum}");

    // 在丢弃error前记录日志
    let lines = ["1", "2", "a"];

    let sum: i32 = lines
        .iter()
        .map(|line| line.parse::<i32>())
        .inspect(|num| {
            if let Err(ref e) = *num {
                println!("Parsing error: {e}");
            }
        })
        .filter_map(Result::ok)
        .sum();

    println!("Sum: {sum}");
}
