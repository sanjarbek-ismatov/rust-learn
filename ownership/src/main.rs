fn main() {
    let mut owned1 = ownership_giver();
    owned1.push('c');
    let mut owned2 = ownership_get_and_back(owned1);
    owned2.push('d');
    println!("{owned2}")
}

fn ownership_get_and_back(mut str: String) -> String {
    str.push('c');
    str
}

fn ownership_giver() -> String {
    let mut str1 = String::from("string1");
    str1.push('a');
    str1
}

fn _ownership_example() {
    let my_name = String::from("Sanjarbek");
    let my_age = 19;
    _new_owner(my_name, my_age);
    println!("My age {my_age}");
    // I cannot access my name here
}

fn _new_owner(s: String, n: i32) {
    println!("My name {s}, my age {n}");
}

fn _reassign_string() {
    let name = String::from("Sanjarbek");
    // name = String::from("Ismatov");
    // this will cause drop function to be called to free the previous memory allocation
    println!("{name}")
}

fn _borrow_example() {
    let name = "Sanjarbek";
    let mut name_modifiable = String::from(name);
    name_modifiable.push_str(" Ismatov");

    let mut borrower = name_modifiable;
    // name_modifiable cannot be used
    borrower.push('a');

    // name_modifiable.push('b'); // it throws an error, because the memory adress is borrowed by another variable

    let mut copier = borrower.clone();
    // you cannot use name_modifiable here anymore

    copier.push_str(" is great!");
    borrower.push('b');
    // it will will not change the copy as they are located differently

    println!(
        "The name hasn't been changed: {name}, but name_modifiable has, borrower is the same as name_modifiable: {borrower}, but copier is different now: {copier}"
    )
}
