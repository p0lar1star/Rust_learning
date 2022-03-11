fn main() {
    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {}", result);

    let mut num = 3;
    while num != 0{
        println!("{}!", num);
        num = num - 1;
    }

    println!("LIFT OFF!");

    // for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is {}", element);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!");
}
