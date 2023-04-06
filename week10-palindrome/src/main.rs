use std::io;

fn main() {
    println!("Enter a string to check if it's a palindrome:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();

    if is_palindrome(input) {
        println!("'{}' is a palindrome!", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}

fn is_palindrome(s: &str) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let len = s.len();

    for i in 0..len / 2 {
        if s[i] != s[len - 1 - i] {
            return false;
        }
    }

    true
}
