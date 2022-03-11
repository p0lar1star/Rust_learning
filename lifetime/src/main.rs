// lifetime-生命周期
// 本代码不可编译，编译不通过
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);
    println!("{}", string2);

    //  For example

    let string1 = String::from("abcd");// string1的生命周期为第14-第25行
    let result;
    {
        let string2 = String::from("xyz");//string2的生命周期为第17-21行
        result = longest(string1.as_str(), string2.as_str());// error:`string2` does not live long enough
        // longest返回的字符串切片的生命周期应为string2的生命周期，即17-21行
        // 但是，result的生命周期为第15-25行，且在24行被使用（即发生了对string2的借用），这是不允许的
    }
    // 到这里时，string2已经离开了作用域，但仍然处于借用的状态
    // 在24行发生了对string2的借用（result
    println!("The longest string is {}", result);
}

// error
// 不知道&str跟哪个生命周期有关
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// right
// 'a表示x和y重叠的那一部分作用域（生命周期
// 换句话说，就是x和y两者中比较短的那个作用域
// 由于返回值也被标注为'a，
// 所以返回值在x和y两者中比较短的那个生命周期内是有效的
// 也就是说返回值的生命周期取x和y中比较短的那一个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
