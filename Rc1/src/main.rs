// 使用Rc需要先导入模块
use std::rc::Rc;
use crate::List::{Cons, Nil};
enum List{
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creatng a is {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creatng b is {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c is {}", Rc::strong_count(&a));
    }
    println!("Now c goes out of scope, {}", Rc::strong_count(&a));
}
