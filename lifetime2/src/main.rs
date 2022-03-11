// 从函数返回引用时，返回类型的生命周期参数必须与函数的某个参数的生命周期匹配
// 是必须!!!!!!↑↑↑↑↑↑↑↑↑↑↑↑
// 如果返回的引用没有指向某个函数参数，那么它只能引用函数内创建的值
// 这就是悬垂引用，因为函数内创建的值在函数结束时就走出了作用域，被销毁了


fn main() {
    let string1= String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // 结构体中引用的生命周期：
    let novel = String::from("Call me Ishmael. Some years ago ...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    println!("{}", first_sentence);// first_sentence: &str
    let i = Astruct {
        part: first_sentence,
    };
    // 为什么可以？因为i这个结构体的生命周期从17行到结束
    // 而first_sentence的生命周期是从15行到结束
    // 结构体内的引用的生命周期必须大于结构体的生命周期！！！
    // 总结：
    // 每个引用都有生命周期，需要为使用生命周期的函数或struct指定生命周期参数
    
}

// error：cannot return reference to local variable `result`
// 因为返回的是函数内部创建的值

// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     let result = String::from("abc");
//     result.as_str()
// }

// 修改：返回String，直接把所有权移交给result

fn longest(x: &str, y: &str) -> String {
    let result = String::from("abc");
    result
}

// 结构体中的引用也需要有生命周期
struct Astruct<'a> {
    part: &'a str,
}

