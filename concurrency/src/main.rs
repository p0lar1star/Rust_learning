// 使用共享来实现并发，继共享内存，多个线程可以访问同一块内存
// Mutex锁
// 通过Mutex::new(数据)来创建Mutex<T>
// 传入的数据就是要保护的数据
// Mutex<T>是一个智能指针
use std::cell::RefCell;
use std::sync::Mutex;
// Rc<T>中的引用计数是单线程引用计数，不是线程安全的
// 而Arc<T>中的引用计数是原子的，是线程安全的
fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        // lock()返回的是MutexGuard类型，指向内部数据，实现了Deref
        // num相当于指向内部数据的可变引用，可以通过解引用访问指向的数据
        *num = 6;
    }// 自动解锁
    println!("{:?}", m);
}