use std::io;

fn main() {
    let mut n = String::new();

    println!("Enter a positive integer:");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input.");

    let n: i64 = n.trim().parse().expect("Please enter a valid integer.");

    let factors = prime_factorize(n);

    println!("Prime factors of {} are: {:?}", n, factors);
}

fn prime_factorize(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let mut n = n;

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    let mut i = 3;
    while i <= (n as f64).sqrt() as i64 {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }

    if n > 2 {
        factors.push(n);
    }

    factors
}
