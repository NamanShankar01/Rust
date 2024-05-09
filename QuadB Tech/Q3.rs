use std::io::{self, Write};

fn shortest_word(s: &str) -> &str {
    let words = s.split_whitespace();
    let mut shortest = "";
    
    for word in words.clone() {
        if word.len() < shortest.len() || shortest.is_empty() {
            shortest = word;
        }
    }
    shortest
}


fn main() {
    let mut input = String::new();
    println!("Enter a string of words: ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let input = input.trim();

    let shortest = shortest_word(&input);
    println!("The shortest word is: {}", shortest);
}
