use rand::Rng;
const NUM: u32 = 1000_0000;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is {}", secret_number);
    let x = 5;
    println!("x is {}", x);
    // shadowing
    let x = x + 1;
    println!("x is {}", x);
    let guess: u32 = "50".trim().parse().expect("Not a number");
    println!("guess is {}", guess);
    println!("Hello, world!");
    println!("The num is {}", NUM);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //点标记法
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    //模式匹配
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    //数组中每个元素类型必须相同
    let a = [1, 2, 3, 4, 5];
    //[类型; 元素个数]
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    //[元素值， 元素个数]
    let c = [3; 5];
    println!("{}, {}, {}", a.len(), b[1], c[4]);
}
