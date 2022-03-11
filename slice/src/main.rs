fn main() {
    // let s = String::from("Hello World!");
    // println!("{}", s);

    // let wordindex = first_word(&s);
    // println!("{}", wordindex);

    //slice切片
    // [开始索引..结束索引]
    // let hello = &s[0..5]; //包括0而不包括5
    // let world = &s[6..11];
    // println!("{}", hello);
    // println!("{}", world);

    // let word = first_word_slice(&s);
    // println!("{}", word);
    // let s = "Hello World!";
    // println!("{}", s);

    let s1 = String::from("Hello slice!");
    let word1 = first_word_slice_modified(&s1[..]);
    let s2 = "Hello slice!";
    let word2 = first_word_slice_modified(s2);
    println!("{}", word1);
    println!("{}", word2);
}

// fn first_word(s: &String) -> usize {
//     // 把String类型转换位字节数组bytes
//     let bytes = s.as_bytes();
//     // iter()方法返回一个数组的迭代器，依次返回数组中的每一个元素
//     // enumerate()方法把iter()返回的结果进行包装，把每个结果作为元组的一部分进行返回
//     // 元组的第一个元素就是enumerate遍历的索引（数组下标）
//     // 元组的第二个元素就是数组元素的引用（这里就是&[u8]）
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// // 返回类型：切片，即&str
// fn first_word_slice(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (j, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..j];
//         }
//     }
//     return &s[..];
// }

//传入参数类型为切片类型，将使得API更通用
fn first_word_slice_modified(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("The length of s is {}", s.len());
            println!("The item is '{}'", item);
            return &s[..i];
        }
    }
    &s[..]
}
