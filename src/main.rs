struct MyString<'a> {
    text: &'a str,//强烈不建议这样使用，结构体标注生命周期会很麻烦，实现方法，关联函数都要标注，看得很烦，而且容易出错，直接使用String
}

impl<'a> MyString<'a> {
    fn get_length(&self) -> usize {
        self.text.len()
    }
    fn modify_text(&mut self) {
        self.text = "world";
    }
}

struct StringHolder {
    detail: String,
}

impl StringHolder {
    fn get_length(&self) -> usize {
        self.detail.len()
    }

    fn get_reference(&self) -> &String {
        &self.detail
    }

    fn get_ref<'a>(&'a self) -> &'a String {
        &self.detail
    }

    fn modify_text(&mut self) -> &mut String {
        &mut self.detail
    }
}


fn main() {
    let mut s = MyString {
        text: "hello",
    };
    println!("{}", s.get_length());
    s.modify_text();
    println!("{}", s.text);

    let n = StringHolder {
        detail: String::from("like you"),
    };
    println!("n: {}", n.get_length());
    println!("n: {}", n.get_reference());
    println!("n: {}", n.get_ref());

    let mut m = StringHolder {
        detail: String::from("like you"),
    };
    let mm = m.modify_text();
    *mm = "not like you".to_owned();
    println!("m: {}", m.get_length());
    println!("m: {}", m.get_reference());
    println!("m: {}", m.get_ref());
}



