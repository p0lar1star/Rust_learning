use std::cell::RefCell;
use std::rc::Rc;
// RefCell<T>
// 提供两个方法（安全接口）
// borrow方法：返回智能指针Ref<T>, 它实现了Deref
// borrow_mut方法：返回智能指针RefMut<T>, 它实现了Deref
// RefCell<T>会记录当前存在多少个活跃的Ref<T>和RefMut<T>智能指针
// 每次调用borrow，不可变借用计数+1，Ref<T>离开作用域被释放时，不可变借用计数-1
// 每次调用borrow_mut，可变借用计数+1，任何一个RefMut<T>离开作用域被释放时，可变借用计数-1
// 在任何一个给定的时间里，只允许拥有多个不可变借用或一个可变借用
// 函数签名如下：
// pub fn borrow(&self) -> Ref<T>
// pub fn borrow_mut(&self) -> RefMut<T>;

// 树结构
// #[derive(Debug)]
// struct Node {
//     value: i32,
//     // 为了能灵活指向，使用RefCell
//     children: RefCell<Vec<Rc<Node>>>,
// }

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<String>>, Rc<List>),
    Nil,
}
use List::{Cons, Nil};

fn main() {
    // let a = RefCell::new(15);
    // let b = a.borrow();
    // let c = a.borrow_mut();// cause panic
    // // error: already borrowed: BorrowMutError
    // println!("b is {}", b);
    // println!("c is {}", c);

    // 结合Rc<T>和RefCell<T>来拥有对可变数据的多个所有者
    let val = Rc::new(RefCell::new(String::from("java")));
    println!("{:#?}", val);
    let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(String::from("C"))), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(String::from("C++"))), Rc::clone(&a));
    // *val -> RefCell<String>
    // *val.borrow_mut() -> RefMut<String>
    *val.borrow_mut() = String::from("C#");
    println!("{:#?}", a);
    println!("{:#?}", b);
    println!("{:#?}", c);
    println!("{:#?}", val);
}
