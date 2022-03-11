// 这是一个库crate
// This is a library crate
// 一个package只能有一个library crate
// One package can only have one crate
// src/lib.rs是library crate的crate root
// crate名和packge名相同，比如这个library crate的名字就是package_crate_module
pub mod front_of_house {
    // 父模块front_of_house
    // hosting是front_of_house的子module
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist!");
        }
    }
}

// // 函数，和front_of_house模块位于同一级
// pub fn eat_at_restaurant() {
//     // 绝对路径，从crate名开始
//     crate::front_of_house::hosting::add_to_waitlist();
//     // 相对路径，能使用是因为front_of_house和eat_at_restaurant处于同一级，所以可以直接调用该模块
//     front_of_house::hosting::add_to_waitlist();
// }

// path
fn serve_order() {
    println!("serve_order!");
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super:上一级
        super::serve_order();
        // crate:根级
        crate::serve_order();
    }
    fn cook_order() {
        println!("cook_order!");
    }

    // public struct
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // 关联函数，即传入的第一个参数不是结构体自身
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("{}", meal.toast);
}
