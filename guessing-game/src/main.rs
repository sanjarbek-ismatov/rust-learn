use rand::Rng;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::stdin;

fn main() {
    println!("Please enter your guess number.");
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("I guess this is an error");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Type a number!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Less => println!("It is low"),
            Equal => {
                println!("You won!");
                break;
            }
            Greater => println!("It is high"),
        }
    }
}
