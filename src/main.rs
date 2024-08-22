trait Draw {
    fn draw(&self) -> Self;
}

#[derive(Clone)]
struct Button;

// self 指代当前的实例对象
// Self 指代Button类型，是类型的别名
impl Draw for Button {
    fn draw(&self) -> Self {
        return self.clone();
    }
}

fn main() {
    let button = Button;
    let newb = button.draw();
}
