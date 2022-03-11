fn main() {
    println!("Hello, world!");
    let y = {
        let z = 1;
        z + 3
    };
    println!("y is {}", y);
    println!("result is {}", plus_five(6));

    let num = 3;
    if num < 5 {
        println!("1");
    }else {
        println!("2");
    }
}
//函数命名用小写字母，常量命名用大写字母

fn plus_five(x: i32) -> i32 {
    x + 5
}