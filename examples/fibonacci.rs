fn fibonacci(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let n = 20;
    println!("fibonacci({}) = {}", n, fibonacci(n));
}
