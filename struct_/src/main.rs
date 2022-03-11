// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         username: username,
//         email: email,
//         sign_in_count: 0,
//         active: true,
//     }
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    // let user1 = User {
    //     email: String::from("123@123.com"),
    //     sign_in_count:556,
    //     active: true,
    //     username: String::from("Nikky"),
    // };

    // //更新语法
    // let user3 = User {
    //     email:String::from("abc@abc.com"),
    //     ..user1// user1 has been moved
    // };

    // let user2 = build_user(String::from("124@124.com"), String::from("Lihua"));
    // println!("User2:{}, {}, {}, {}", user2.username, user2.email, user2.sign_in_count, user2.active);
    // println!("User3:{}, {}, {}, {}", user3.username, user3.email, user3.sign_in_count, user3.active);
    //caculate the area
    // let rect = (30, 50);
    // println!("{}", area(rect));

    let rect = Rectangle {
        width: 300,
        length: 500,
    };
    println!("{}", area_1(&rect));

    println!("{:#?}", rect);
}

// fn area(dim: (u32, u32)) -> u32 {
//     dim.0 * dim.1
// }

fn area_1(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}
