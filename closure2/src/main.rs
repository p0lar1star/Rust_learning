// 闭包捕获上下文环境变量
fn main() {
    let x = 4;
    let equal_to_x = |z: i32| -> bool {
        z == x
    };
    // fn z_equal_to_x(z: i32) -> bool {
    //     z == x// fn定义的函数不能捕获外部变量can't capture dynamic environment in a fn item
    // }
    let y = 4;
    assert!(equal_to_x(y));
}
