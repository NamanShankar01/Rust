use std::io::{self, Write};

fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    let mut prefix = strs[0].clone();
    for s in &strs[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return "".to_string();
            }
        }
    }
    prefix
}

fn main() {
    let mut input = String::new();
    println!("Enter a set of strings (space separated): ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let strs: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

    let lcp = longest_common_prefix(&strs);
    println!("The longest common prefix is: {}", lcp);
}
