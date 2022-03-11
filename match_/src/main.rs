#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Mickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(c));

    let v = Some(3u8);
    match v {
        Some(3) => println!("three1"),
        _ => (), // _匹配其他所有情况，否则error
    }

    // if let
    // 只匹配一种情况
    // 放弃了穷举的可能
    if let Some(3) = v {
        println!("three2");
    } else {
        println!("others");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    //match匹配时要穷举所有的模式
    //否则，使用_通配符代替其余没列出的值
    match coin {
        Coin::Penny => {
            println!("Penny!");
            1
        }
        Coin::Mickel => 5,
        Coin::Dime => 10,
        // 绑定值的模式匹配
        Coin::Quarter(state) => {
            println!("The state is {:#?}", state);
            25
        }
    }
}
