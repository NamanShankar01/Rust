use std::io::{self, Write};

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    println!("Enter a number: ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let n: i32 = input
                    .trim()
                    .parse()
                    .unwrap();

    if is_prime(n) {
        println!("{} is a prime number", n);
    } else {
        println!("{} is not a prime number", n);
    }
}
