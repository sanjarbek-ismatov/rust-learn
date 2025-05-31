use std::io::stdin;

fn main() {
    println!("Please enter your guess number: ");
    let mut guess = String::new();
    stdin()
        .read_line(&mut guess)
        .expect("I guess this is an error");
    println!("Your guess {guess}");
}
