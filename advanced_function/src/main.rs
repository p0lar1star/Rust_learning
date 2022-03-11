// 函数指针
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 返回闭包：编译器不知道函数的大小，无法直接返回
// 但可以返回指针
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
fn main() {
    let ans = do_twice(add_one, 5);
    println!("{}", ans);
}
