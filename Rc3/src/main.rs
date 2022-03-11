// 为防止循环引用，把Rc<T>换成Weak<T>
// Rc<T>实例通过调用Rc::downgrade方法可以创建值的Weak Reference（弱引用）
// 返回类型是Weak<T>(智能指针)
// 调用Rc::downgrade会为weak_count +1
// Rc<T>使用weak_count来追踪存在多少Weak<T>
// weak_count不为0并不影响Rc<T>实例的清理
// 在使用Weak<T>之前，需保证它指向的值仍然存在,即：
// 在Weak<T>实例上调用upgrade方法，返回Option<Rc<T>>

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// 尝试创建一棵树
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>, // 子节点保存在vector中，用智能指针指向子节点，共享子节点所有权
    parent: RefCell<Weak<Node>>,
}
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]), // 叶子结点的子节点为空
        parent: RefCell::new(Weak::new()),
    });
    // 通过子节点访问父节点，暂时为空
    // Weak::upgrade()返回Rc<T>
    println!("leaf's parent is {:#?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });
    // 父节点访问子节点
    println!("{}", (*branch.children.borrow())[0].value);
    // 子节点指向父节点
    // Rc::downgrade()返回Weak<T>
    *(leaf.parent.borrow_mut()) = Rc::downgrade(&branch);
    println!("leaf's parent is {:#?}", leaf.parent.borrow().upgrade());
}
