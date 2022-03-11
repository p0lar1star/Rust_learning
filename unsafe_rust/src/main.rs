// unsafe超能力：1~4
// 1.解引用原始指针
// 2.调用unsafe函数或方法
// 3.访问或修改可变的静态变量
// 4.实现unsafe trait
// 但是，unsafe并没有关闭借用检查或停用其他安全检查
// 尽可能隔离unsafe代码，最好将其封装在安全的抽象里，提供安全的API
use std::slice;
static HELLO_WORLD: &str = "HELLO WORLD!";
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn main() {
    println!("{}", HELLO_WORLD);
    // 访问和修改可变的静态变量是不安全的操作
    add_to_counter(3);
    unsafe {
        println!("COUNTER:{}", COUNTER);
    }
    // let mut num = 5;
    // raw_pointer
    // 与引用不同，原始指针允许通过同时具有不可变和可变指针或多个指向同一位置的可变指针来忽略借用规则
    // 原始指针无法保证能指向合理的内存
    // 原始指针允许位Null
    // 原始指针不实现任何自动清理
    // 可以在任何地方创建可变或不可变的原始指针
    // 但访问其指向的内容必须要在unsafe块中
    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;
    // unsafe {
    //     println!("{}", *r1);
    //     println!("{}", *r2);
    //     *r2 += 1;
    // }
    // println!("{}", num); // 值被改变

    // 内存地址
    // let address = 0x7fff8780usize;
    // let r = address as *const i32;
    // unsafe {
    //     println!("r:{}", *r); // Segmentation fault
    // }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (front, back) = v.split_at_mut(3);
    println!("{:?}", front);
    println!("{:?}", back);
    println!("{:?}", v);
}

// 不安全函数，只能在unsafe块中被调用
unsafe fn dangerous() {}

// 在安全的函数中使用不安全的代码块
// fn spilt_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {}


// unsafe trait
unsafe trait Foo {}

unsafe impl Foo for i32 {}
