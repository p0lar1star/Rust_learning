fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third num is {}", third);
    // 使用get，越界时返回None
    match v.get(2) {
        Some(third) => println!("The third num is {}", third),
        None => println!("There is no third num"),
    }
    // 遍历
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}
