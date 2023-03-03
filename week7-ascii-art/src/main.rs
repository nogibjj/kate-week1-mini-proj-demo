use figlet_rs::FIGfont;

fn main() {
    let mut input = String::new();
    println!("Enter a string to convert to ASCII art: ");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(input);
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}