fn main() {
    fn no_return(n: i32) -> i32 {
        n
    }
    fn with_return(mut n: i32) -> i32 {
        n = n / 2;
        return n;
    }
    let a = {
        let b = 25 - 20;
        b + 10
    };
    println!("{a} {} {}", no_return(10), with_return(15))
}
