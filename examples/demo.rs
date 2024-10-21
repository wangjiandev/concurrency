fn main() {
    let input = '?';
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase letter {}", key),
        key if key.is_uppercase() => println!("Uppercase letter {}", key),
        _ => println!("Something else"),
    }
}
