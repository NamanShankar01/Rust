fn main() {
    let mut input = String::new();
    println!("Enter a string: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut chars: Vec<char> = input.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();

    if is_palindrome(&chars) {
        println!("{input} is a palindrome");
    } 
    else {
        println!("{input} is not a palindrome");
    }
}

fn is_palindrome(input: &Vec<char>) -> bool {
    let mut i = 0;
    while i < (input.len() / 2) {
        if input[i] != input[input.len() - 1 - i] {
            return false; 
        }
        i += 1;
    }
    true
}