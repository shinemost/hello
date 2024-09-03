fn f<'a, T>(x: *const T) -> &'a T {
    unsafe {
        // 此时野指针凭空产生a生命周期,
        &*x
    }
}

// b>=a
struct DoubleRef<'a, 'b: 'a, T> {
    r: &'a T,
    s: &'b T,
}
// a<=T
struct Ref<'a, T> {
    r: &'a T,
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 此处必须指定a:b slef.part的生命周期是a
// 而方法的返回值生命周期是b
// 只有a>=b才能将a转为b
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn main() {}