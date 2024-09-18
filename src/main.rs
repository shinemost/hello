// 自引用结构体
struct SelfRef<'a> {
    value: String,

    // 该引用指向上面的value
    pointer_to_value: &'a str,
}

#[derive(Debug)]
struct WhatAboutThis<'a> {
    name: String,
    nickname: Option<&'a str>,
}

// fn creator<'a>() -> WhatAboutThis<'a> {
//     let mut tricky = WhatAboutThis {
//         name: "Annabelle".to_string(),
//         nickname: None,
//     };
//     tricky.nickname = Some(&tricky.name[..4]);

//     tricky
// }
impl<'a> WhatAboutThis<'a> {
    fn tie_the_knot(&'a mut self) {
        self.nickname = Some(&self.name[..4]);
    }
}
fn main() {
    // let s = "aaa".to_string();
    // let v = SelfRef {
    //     value: s,
    //     pointer_to_value: &s,
    // };

    // let mut tricky = WhatAboutThis {
    //     name: "Annabelle".to_string(),
    //     nickname: None,
    // };
    // tricky.nickname = Some(&tricky.name[..4]);

    // println!("{:?}", tricky);

    let mut tricky = WhatAboutThis {
        name: "Annabelle".to_string(),
        nickname: None,
    };
    tricky.tie_the_knot();

    println!("{:?}", tricky);
}
