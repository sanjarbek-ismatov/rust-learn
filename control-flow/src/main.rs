use std::io::stdin;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error in input");
    let age: u8 = input.trim().parse().expect("Error in parsing");
    let is_adult = if age < 18 { true } else { false };
    if age < 18 {
        println!("Get older, then comeback!, is_adult: {is_adult}")
    } else {
        println!("There you go, cowboy!")
    }
}
