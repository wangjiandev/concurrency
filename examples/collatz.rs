fn collatz_length(mut n: u32) -> u32 {
    let mut length = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        length += 1;
    }
    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_length() {
        assert_eq!(collatz_length(1), 1);
        assert_eq!(collatz_length(2), 2);
        assert_eq!(collatz_length(3), 8);
        assert_eq!(collatz_length(4), 3);
        assert_eq!(collatz_length(5), 6);
        assert_eq!(collatz_length(11), 15);
    }
}

fn main() {
    println!("{}", collatz_length(1));
    println!("{}", collatz_length(2));
    println!("{}", collatz_length(3));
    println!("{}", collatz_length(4));
    println!("{}", collatz_length(5));
    println!("{}", collatz_length(11));
}
