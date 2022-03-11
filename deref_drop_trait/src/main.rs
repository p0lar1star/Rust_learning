// 要使用Deref trait，必须首先
use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        println!("Now creating a MyBox");
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn hello(string: &str) {
    println!("Hello, {}", string);
}

struct CustomSmartPointer {
    data: String,
}

// 实现Drop trait
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data:{}", self.data);
    }
}

fn main() {
    // Deref trait
    let x = 5;
    let y = MyBox::new(5);
    assert_eq!(x, 5);
    assert_eq!(*y, 5);
    // 自动解引用
    let m = MyBox::new(String::from("Rust"));
    // &m: &MyBox<String>
    // deref: &String
    // deref: &str
    // rust中，为&T与&mut T (T为任意类型)实现了Deref_trait
    // 传入hello函数，参数类型不符合，自动调用&m的deref方法
    // 而任意&T的deref的方法体为*self,即返回T类型
    // 随后继续deref，转为String，不符合，继续，转为&str
    hello(&m);


    // Drop trait
    let c = CustomSmartPointer{data: String::from("I'm C")};
    let d = CustomSmartPointer{data: String::from("I'm D")};
    println!("Now program quit");
}
