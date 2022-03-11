// 为自己的结构体实现自定义的迭代器
// 实际上只有一步：提供next()方法的实现
#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 实现自定义迭代器：为Counter结构体实现Iterator trait
impl Iterator for Counter {
    type Item = u32;
    // 可以理解为Option<u32>
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // let mut a = [1, 2, 3, 4, 5];
    // for &i in &a {
    //     println!("{}", i);
    // }
    // a[4] = 6;
    // println!("\n{}", a[4]);
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // 跳过第一个元素，zip返回的是元组形式的迭代器
        .map(|(a, b)| a * b) // 把元组中两个元素相乘，得到数组形式的迭代器
        .filter(|x| x % 3 == 0)// 过滤掉不是三的倍数的数，得到数组形式的迭代器
        .sum();

    println!("{:#?}", sum);
}
