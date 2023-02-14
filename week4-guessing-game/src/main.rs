use rand::thread_rng;
use rand::distributions::Distribution;
use std::io;

fn main() {
    // Generate a random number between 1 and 100
    let secret_number = rand::distributions::Uniform::from(1..=100).sample(&mut thread_rng());

    let mut guess = String::new();

    println!("Guess the number!");
    loop {
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
