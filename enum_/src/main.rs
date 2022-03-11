// 枚举类型IpAddrKind有两种值V4和V6
#[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr_ {
//     kind: IpAddrKind,
//     address: String,
// }

// better
// 优点：不需要额外使用struct
// 每个变体可以拥有不同类型以及关联的数据量
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 消息的枚举类型
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 匿名的结构体
    Write(String),
    ChangeColor(i32, i32, i32),
}

//定义枚举中的方法
impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

fn main() {
    // let home = IpAddr_ {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr_ {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    //初始化枚举变量
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);

    m.call();
}
// fn route(ip_kind: IpAddrKind) {
//     println!("{:#?}", ip_kind);
// }
