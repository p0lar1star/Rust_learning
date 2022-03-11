use std::thread;
use std::time::Duration;
// 消息传递通道
use std::sync::mpsc;

fn main() {
    // 使用thread::spawn来创建新线程
    // 若不在主线程中使用join方法进行阻塞，则无论新线程是否执行完，都将退出
    // 向spawn()传入闭包，闭包就是要在新县城中执行的函数
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("nuber {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 阻塞
    handle.join().unwrap();

    // move关键字，使得主线程中v的所有权被转移到新线程中
    // 否则，编译不通过，因为新线程无法确定：万一v在主线程中已经被drop掉了呢
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("vector: {:?}", v);
    });
    handle.join().unwrap();

    // channel：多个生产者，单个消费者的线程消息传递通道
    // 返回一个元组，前者为发送者，后者为接受者
    let (tx, rx) = mpsc::channel();
    // 克隆一个发送者tx1
    let tx1 = mpsc::Sender::clone(&tx);

    // tx发送
    thread::spawn(move || {
        let vals = vec![
            String::from("tx: hi"),
            String::from("tx: from"),
            String::from("tx: the"),
            String::from("tx: thread"),
        ];
        for val in vals {
            // 一旦发送，val失去所有权
            tx.send(val).unwrap();
            // println!("{}", val);// error
            thread::sleep(Duration::from_millis(200));
        }
    });

    // tx1发送
    thread::spawn(move || {
        let vals = vec![
            String::from("tx1: hi"),
            String::from("tx1: from"),
            String::from("tx1: the"),
            String::from("tx1: thread"),
        ];
        for val in vals {
            // 一旦发送，val失去所有权
            tx1.send(&val).unwrap();
            // println!("{}", val);// error
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got {}", received);
    }
}
