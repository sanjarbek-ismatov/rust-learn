pub fn fibonacci_gen(num: i32) -> i32 {
    if num < 1 {
        return 0;
    };
    let mut calculated_num = 1;
    for current_num in 1..=num {
        calculated_num *= current_num;
    }
    calculated_num
}
