use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is {}", secret_number);
    loop {
        println!("Inout a number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Can't read a line.");
        //shadow
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your input number is {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater=> println!("Too big"),
            Ordering::Equal=> {
                println!("You win!");
                break;
            }
        }
    }

}