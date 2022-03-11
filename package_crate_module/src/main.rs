// 这是一个二进制crate
// This is a binary crate
// crate名与package名相同
// 一个package可以包含多个二进制crate，但必须至少有一个二进制crate
// src/bin目录下的每个文件都是一个单独的crate
// src/main.rs是binary crate的crate root
use package_crate_module::front_of_house;
use package_crate_module::eat_at_restaurant;//lib.rs
fn main() {
    eat_at_restaurant();
    front_of_house::hosting::add_to_waitlist();
    println!("Hello, world!");
}
