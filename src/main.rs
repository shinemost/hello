fn f<'a, T>(x: *const T) -> &'a T {
    unsafe {
        // 此时野指针凭空产生a生命周期,
        &*x
    }
}

fn main() {}