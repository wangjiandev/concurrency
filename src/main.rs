use std::str::FromStr;

use num_bigint::BigInt;

fn main() {
    let mut begin = String::from("6");
    let mut sum = BigInt::ZERO;
    for _ in 1..=100 {
        begin.push('9');
        let a = BigInt::from_str(&begin).unwrap();
        sum += a;
        println!("{}", begin);
        println!("={}", sum);
    }
}
