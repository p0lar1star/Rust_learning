// 泛型 trait和生命周期
// 使用泛型的代码和使用具体类型的代码运行速度是一样的，原因：单态化
// 寻找vector中的最大数
// fn largest(list: &[i32]) -> i32 {
//     let mut largest_num = list[0];
//     for &num in list {
//         if num > largest_num {
//             largest_num = num;
//         }
//     }
//     largest_num
// }
//泛型
//
// struct Point<T> {
//     x: T,
//     y: T,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// enum Option<T> {
//     Some(T),
//     None,
// }
// 为struct或enum实现方法时，可在定义中使用泛型
// 把T放在impl关键字后，表示在类型T上实现方法
// impl<T> Point<T> {
//     fn x1(&self) -> &T {
//         &self.x
//     }
// }
// // 只针对具体类型
// impl Point<f64> {
//     fn f1(&self) -> &f64 {
//         &self.x
//     }
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let p1 = Point { x: 5, y: 4 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
}
