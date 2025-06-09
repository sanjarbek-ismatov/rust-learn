fn main() {
    const NAME: &str = "Sanjarbek";
    let age: u32 = 19;
    println!("Hello, {NAME}, your age is {age}!");
    let height: u8 = 179u8;
    println!("Height as an integer {height}");
    let height: f32 = 179.5f32; // I explicitly mentioned f32, otherwise it's f64 
    println!("Height as a float number {height}");
    let exam_bin = 0b100110; // 38
    println!("bin: {exam_bin}");
    let truncated = 11 / 2; // It is an integer, not float
    let precised = 11.0 / 2.0;
    println!("{truncated} {precised}");
    let mut is_married = false;
    let mut answer = String::new();
    std::io::stdin()
        .read_line(&mut answer)
        .expect("Error occured during the process!");
    if answer.trim() == "yes" {
        is_married = true;
    }
    println!("Is Married? {is_married}");
    let heart_eyed_cat = '😻';
    println!("Rust's most primitive data type: {heart_eyed_cat}")
    // name = "Changed";
}
