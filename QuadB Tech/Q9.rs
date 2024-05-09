use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    println!("Enter a string: ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let input = input.trim();

    let reversed: String = input.chars().rev().collect();
    println!("The reversed string is: {}", reversed);
}
