use std::io::stdin;
fn main() {
    for i in (1..=10).rev() {
        print!("{i} ")
    }
}

fn _stars_print() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error in input");
    let rows_total: u8 = input.trim().parse().expect("Error occurred");
    let mut row_count = 0;
    'outer_loop: loop {
        let mut col_count: u8 = 0;
        if row_count <= rows_total {
            'inner_loop: loop {
                if col_count <= row_count {
                    print!("*");
                    col_count += 1;
                } else {
                    break 'inner_loop;
                }
            }
            println!();
            row_count += 1;
        } else {
            break 'outer_loop;
        }
    }
}

fn _age_verifier() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error in input");
    let age: u8 = input.trim().parse().expect("Error in parsing");
    let is_adult = if age < 18 { true } else { false };
    let result = loop {
        if is_adult {
            break "You are too young";
        } else {
            println!("There you go, cowboy!");
        }
    };
    println!("Here is the result: {result}")
}
