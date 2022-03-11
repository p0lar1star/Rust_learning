fn main() {
    // println!("Hello World!");

    // function
    // another_function(5, 6);
    // let x = 5;
    // println!("original x is {}", x);
    // let y = {
    //     let x = 1;
    //     x + 3
    // };
    // println!("y is {}", y);
    // let x = five();
    // println!("x is {}", x);
    // let z = plus_five(6);
    // println!("z is {}", z);

    // condition
    // let number = 3;
    // if number < 5 {
    //     println!("condition was true!");
    // } else {
    //     println!("condition was false!");
    // }
    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // println!("The value of number is {}", number);

    //match

    //for
    // let a = [10, 20, 30, 40, 50];
    // for element in a.iter() {
    //     println!("the value is {}", element);
    // }

    //for, Range, rev()
    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFT OFF!");

    // //String
    // let mut s = String::from("Hello"); //heap
    // s.push_str(", World!");
    // println!("{}", s);
    // //move
    // //s1被移动了，移动后s1不再有效
    // let s1 = String::from("Hello"); // s1->ptr point to a heap addr
    // let s2 = s1; // s2->ptr point to heap addr that s1->ptr point to, s1 has been moved
    // println!("{}", s2);
    // // println!("{}", s1); // false

    // //clone
    // let s1 = String::from("Hello");
    // let s2 = s1.clone();
    // println!("{}, {}", s1, s2);

    //copy trait
    //drop trait

    // //将值传递给函数会发生移动或复制，取决于传递的指的类型
    // //如果传递的值实现了copy trait则发生的是复制，比如i32证书类型
    // //如果传递的值没实现copy trait，则发生的是移动，比如String类型
    // //移动后，原来的值不再有效
    // let s = String::from("Hello somestring");
    // take_ownership(s);
    // let x = 5;
    // makes_copy(x);
    // println!("x:{}", x);

    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2); // s2 has been moved
    //                                    // then s1 and s3 will be dropped
    // println!("{}, {}", s1, s3);

    // //引用和借用
    // // 把引用作为函数参数的行为叫做借用
    // let mut s1 = String::from("hello");
    // println!("The length of s1 is {}", s1.len());
    // let len = calculate_length(&mut s1);
    // println!("The length of '{}' is {}", s1, len);
    // // 同一作用域内不可拥有多个可变引用(防止条件竞争)形式类似于&mut String
    // // 不同作用域内可以非同时的有多个可变引用
    // let mut s = String::from("Hello");
    // {
    //     let s1 = &mut s;
    //     // 到s2时，s1已经出作用域了，不再有效
    // }
    // let s2 = &mut s;
    // // 可拥有多个不可变引用
    // // 不可同时拥有可变引用和不可变引用
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // // s.push_str(", world!"); // error
    // println!("{}, {}", r1, r2);
    
    // dangling references
    // Rust编译器保证应用永远都不是悬空引用
    // 如果引用了某些数据，编译器将保证引用离开作用域前数据不会离开作用域
    // let r = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     // &s // error
// }

// fn calculate_length(s: &mut String) -> usize {
//     s.push_str(", world!");
//     s.len()
// }

// //给String一个所有权
// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string // 返回值的所有权的移动
// }

// // 取得一个String的所有权，并将它返回
// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn take_ownership(some_string: String) {
//     println!("{}", some_string);
//     // Rust自动调用drop函数，some_string被释放
// }

// fn makes_copy(some_number: i32) {
//     println!("{}", some_number);
//     // 不会发生什么特别的事，因为some_number在栈上
// }

// fn another_function(x: i32, y: i64) {
//     println!("Another Function!");
//     println!("The number is {} and {}", x, y);
// }

// fn five() -> i32 {
//     5
// }

// fn plus_five(x: i32) -> i32 {
//     x + 5
// }
