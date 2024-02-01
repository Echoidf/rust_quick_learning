#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}


#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("function call...");
    }
}


// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    println!("{:?}", home);

    let msg = Message::Write(String::from("Hello"));
    msg.call();

    // rust 标准库中的枚举 Option<T> {Some(T), None}
    let s : Option<String> = Some(String::from("hello"));
    println!("{}", s.is_some());
}
