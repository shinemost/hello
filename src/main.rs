fn first(arr: &[i32]) -> Option<&i32> {
    arr.get(6)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    let r = first(&a);
    println!("{:?}", r);

    let s = "";
    let m = last_char_of_first_line(&s).expect("空字符串");
    println!("{m}");
}
