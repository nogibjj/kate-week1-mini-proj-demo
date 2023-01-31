# Week1 Mini Proj Demo - A simple CLI Tool that greets people

## Usage
This CLI tool takes in two arguments, `--firstname <FIRSTNAME>` and `--lastname <LASTNAME>`, and it outputs a greeting message. For example, if I type in command line `cargo run -- hello --firstname "Kate" --lastname "Feng"`, the output message is "Hello Kate Feng! Have a wonderful day!"

## Implementation
This project is built on top of the rust-cli-template that's provided, and implemented following the structure of the Marco Polo example discussed in class. There are two files, lib.rs and main.rs. Lib.rs contains the hello function that takes in two strings and returns a concatenated name and greeting. Main.rs contains clap package set up and uses the hello function in lib.rs.

## Conclusion
This is a simple but practical example of CLI tool project. The main purpose is to get familiar with Rust and codespace.

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
