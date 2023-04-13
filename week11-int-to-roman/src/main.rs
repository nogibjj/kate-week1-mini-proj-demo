use std::io;

fn main() {
    loop {
        println!("Choose an option:");
        println!("1. Convert integer to Roman numeral");
        println!("2. Convert Roman numeral to integer");
        println!("3. Quit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter an integer that is greater than 0:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let input = input.trim().parse::<u32>().unwrap_or(0);
                if input < 1 {
                    println!("Invalid input. Please enter a number that is greater than 0");
                    continue;
                }
                println!("Roman numeral: {}", int_to_roman(input));
            }
            "2" => {
                println!("Enter a Roman numeral:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let input = input.trim();
                match roman_to_int(input) {
                    Some(value) => println!("Integer: {}", value),
                    None => println!("Invalid Roman numeral."),
                }
            }
            "3" => break,
            _ => println!("Invalid choice. Please choose a valid option."),
        }
    }
}

fn int_to_roman(num: u32) -> String {
    let mut result = String::new();
    let mut remaining = num;

    let roman_values = [
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    for &(symbol, value) in roman_values.iter() {
        while remaining >= value {
            result.push_str(symbol);
            remaining -= value;
        }
    }

    result
}

fn roman_to_int(s: &str) -> Option<u32> {
    let mut result = 0;
    let mut prev_value = std::u32::MAX;

    let roman_values = [
        ('M', 1000),
        ('D', 500),
        ('C', 100),
        ('L', 50),
        ('X', 10),
        ('V', 5),
        ('I', 1),
    ];

    for c in s.chars() {
        let value = match roman_values.iter().find(|&&(symbol, _)| symbol == c) {
            Some(&(_, value)) => value,
            None => return None,
        };

        if value > prev_value {
            result += value - 2 * prev_value;
        } else {
            result += value;
        }

        prev_value = value;
    }

    Some(result)
}
