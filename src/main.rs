/*
 * @LastEditors: shinemost supertain147@163.com
 * @Date: 2024-06-05 22:36:45
 * @LastEditTime: 2024-06-09 11:12:50
 * @FilePath: /hello/src/main.rs
 */
fn main() {
    let drink = Drink {
        flavor: Flavor::Sweet,
        price: 6.0,
    };
    println!("{}", drink.price);
    // print_drink(drink);//所有权转移
    drink.buy();
    let drink = Drink::new(12.0);
    drink.buy();
}

enum Flavor {
    Sweet,
    Fruity,
    Splicy,
}

struct Drink {
    flavor: Flavor,
    price: f64,
}

impl Drink {
    // 关联变量
    const MAX_PRICE: f64 = 10.0;
    //方法
    fn buy(&self) {
        if self.price > Drink::MAX_PRICE {
            println!("I am poor");
            return;
        }
        println!("buy it");
    }

    //关联函数
    fn new(price: f64) -> Self {
        Drink {
            flavor: Flavor::Fruity,
            price,
        }
    }
}
// 普通函数
fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sweet => println!("sweet"),
        Flavor::Splicy => println!("splicy"),
        Flavor::Fruity => println!("fruity"),
    }
    println!("{}", drink.price);
}
