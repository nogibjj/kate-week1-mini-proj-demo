use std::collections::HashMap;
use std::io;

fn morse2english(morse_code: HashMap<char, &str>){
    //enter morse code to translate to english
    println!("Enter morse code to translate to English:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut output = String::new();
    //go through a string of morse code and translate it to english
    //split the string by whitespace
    for c in input.split_whitespace() {
        //go through the hashmap and find the key that matches the value
        for (key, value) in &morse_code {
            if c == *value {
                output.push(*key);
            }
        }
    }
    println!("{}", output);
}

fn english2morse(morse_code: HashMap<char, &str>){
    println!("Enter a message to translate to Morse code:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut output = String::new();
    for c in input.to_lowercase().chars() {
        if let Some(code) = morse_code.get(&c) {
            output.push_str(code);
            output.push(' ');
        }
    }
    println!("{}", output);
}

fn main() {
    let morse_code: HashMap<char, &str> = [
        ('a', ".-"), ('b', "-..."), ('c', "-.-."), ('d', "-.."), ('e', "."), ('f', "..-."), ('g', "--."), ('h', "...."),
        ('i', ".."), ('j', ".---"), ('k', "-.-"), ('l', ".-.."), ('m', "--"), ('n', "-."), ('o', "---"), ('p', ".--."),
        ('q', "--.-"), ('r', ".-."), ('s', "..."), ('t', "-"), ('u', "..-"), ('v', "...-"), ('w', ".--"), ('x', "-..-"),
        ('y', "-.--"), ('z', "--.."), ('0', "-----"), ('1', ".----"), ('2', "..---"), ('3', "...--"), ('4', "....-"),
        ('5', "....."), ('6', "-...."), ('7', "--..."), ('8', "---.."), ('9', "----."), (' ', "/")
    ].iter().cloned().collect();

    //have a loop to ask user if they want to translate to morse code or english
    //if input is 1, call english2morse function
    //if input is 2, call morse2english function
    //if input is invalid, print "Invalid input"
    //if input is 3, exit the program

    loop {
        println!("Welcome to the Morse Code Translator! Please enter\n1 to translate from English to Morse code,\n2 to translate Morse Code to English,\n3 to exit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: u32 = input.trim().parse().expect("Please type a number!");
        if input == 1 {
            english2morse(morse_code.clone());
        } else if input == 2 {
            morse2english(morse_code.clone());
        } else if input == 3 {
            break;
        } else {
            println!("Invalid input");
        }
    }

}
