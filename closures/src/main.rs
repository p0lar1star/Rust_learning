use std::time::Duration;
use std::thread;
// 闭包：可以捕获其所在环境的匿名函数

// 结构体中存储闭包和闭包的返回值
// 即所谓的“缓存器”，存储需要的值和得到该值的闭包，若值为空则使用闭包计算
struct Cacher<T>
    where T: Fn(u32) -> u32,// 闭包传入的参数类型为u32，返回类型为u32
{
    caculation: T,// caculation就是闭包
    value: Option<u32>,// 闭包的返回值
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32,
{
    // 构造函数
    fn new(caculation: T) -> Cacher<T> {
        Cacher { caculation, value: None, }
    }

    // 获得value值
    // 若value不存在，则运行闭包函数，修改value值
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.caculation)(arg);
                self.value = Some(v);// 执行结果封装到Some里面，返回给value
                v
            }
        }
    }

}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);

    // 下面代码与本程序主要功能无关
    // 闭包会自动推断类型，如果你不给他指定类型
    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));// x被推断为String
    // let n = example_closure(5);// 报错，x已经被推断为String，不能是u32

}

fn generate_workout(intensity: u32, random_number: u32) {
    // 定义闭包函数，相当于把函数的定义给该变量expensive_closure，没有执行函数！
    // let expensive_closure = |num: u32| {// num是这个闭包函数的参数
    //     // 假定这个函数时间开销很大，用睡眠两秒代替
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };// 注意有分号
    let mut expensive_closure = Cacher::new(|num: u32| {// num是这个闭包函数的参数
        // 假定这个函数时间开销很大，用睡眠两秒代替
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));// value方法可能会对expensive_closure结构体进行修改，故要将expensive_closure声明为可变的(mut)
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remenber to stay hydrated!");
        } else {
            println!("Today, run {} minutes!", expensive_closure.value(intensity));
        }
    }
}