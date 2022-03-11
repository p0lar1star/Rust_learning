// trait bound
fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> T{
    let mut largest_num = list[0];
    for &item in list {
        if item > largest_num {// std::cmp::PartialOrd
            largest_num = item;
        }
    }
    largest_num
}


fn largest_string(string: &Vec<std::string::String>) -> String{
    let mut largest_str = string[0].clone();
    // println!("{}", largest_str);
    for i in string {
        if *i > largest_str {
            largest_str = i.clone();
        }
    }
    largest_str.clone()
}

fn largest_string_Type<T: Clone + std::fmt::Display + PartialOrd>(string: &Vec<T>) -> T {
    let mut largest_str = string[0].clone();
    // println!("{}", largest_str);
    for i in string {
        if *i > largest_str {
            largest_str = i.clone();
        }
    }
    largest_str.clone()
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest_string_Type(&str_list);
    println!("The largest string is {}", result);
}
