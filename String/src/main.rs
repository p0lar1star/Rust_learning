fn main() {
    // println!("Hello, world!");
    // // 创建空字符串
    // let mut s = String::new();
    // s.push_str("😇🥰👁️");
    // println!("{}", s);
    // let t = "😭".to_string();
    // println!("{}", t);
    // // push_str()把一个字符串切片附加到String（不会拿走所有权）
    // // push()把单个字符附加到String
    // // +拼接
    // let s1 = String::from("Hello");
    // let s2 = String::from(" World!");

    // let s3 = s1 + &s2;//s1从此失效，+取得了s1的所有权
    // println!("{}", s3);
    // println!("{}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s3 = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s3);
    // println!("{}", s2);
    // println!("{}", s1);// error


    // format!
    // 直接按指定格式连接字符串
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s4);
    println!("{}", s3);
    println!("{}", s2);
    println!("{}", s1);


    // String不能通过数组下标访问
    let s = String::from("🥰🥰🥰");
    println!("{}", s.len());// len()返回的是字节长度，长为12个字节


    // 以字节形式看待字符串
    let s = String::from("🥰👀👁️");
    for b in s.bytes() {
        println!("{}", b);
    }


    //以utf-8标量值的形式来看待字符串
    let c = String::from("🥰👀👁️");
    for b in c.chars() {
        println!("{}", b);
    }


    // 以字形簇
}
