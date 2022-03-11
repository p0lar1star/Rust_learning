fn main() {
    let x = Some(5);
    let y = 10;

    // | 表示或
    match x {
        Some(5) | Some(50) => println!("Got 5 or 50"),
        // y作用域仅在match内，模式匹配Some(y) = Some(5)，所以y = 5
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Defalut case, x = {:?}", x),
    }
    println!("at the end, x = {:?}, y = {:?}", x, y);

    // 范围匹配
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("Something else"),
    }
}
