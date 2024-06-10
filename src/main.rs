/*
 * @LastEditors: shinemost supertain147@163.com
 * @Date: 2024-06-05 22:36:45
 * @LastEditTime: 2024-06-10 12:28:32
 * @FilePath: /hello/src/main.rs
 */
fn main() {
    let age = 90;

    // if的表达能力很弱，只适合简单的场景，if嵌套会影响可读性
    if age > 50 {
        println!("you are old");
    } else {
        println!("you are young");
    }

    let num = 90;
    if num > 90 {
        println!(" very good");
    } else if num > 60 {
        println!("good");
    } else {
        println!("bad");
    }

    let msg = if num > 90 {
        "excellent".to_owned()
    } else {
        "good".to_owned()
    };
    println!("you are {msg}");

    // match 表达性很强，适用于复杂业务场景，可以与if搭配使用
    match num {
        80 => println!("good"),
        90 => println!("excellent"),
        _ => println!("Some else"),
    }

    match num {
        60..=100 => println!("good"),
        20..=59 => println!("bad"),
        _ => println!("some else"),
    }

    match num {
        25 | 50 | 75 => println!("25 or 50 or 75"),
        80 | 100 => println!("80 or 100"),
        _ => println!("some else"),
    }

    match num {
        x if x > 90 => println!("excellent"),
        x if x > 60 => println!("good"),
        _ => println!("some else"),
    }
    // if 和match都可以作为表达式返回
    let msg = match num {
        x if x > 90 => "excellent".to_owned(),
        x if x > 60 => "good".to_owned(),
        _ => "some else".to_owned(),
    };
    println!("you are {msg}");
}
