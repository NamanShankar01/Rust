use std::io::{self, Write};

fn median(arr: &[i32]) -> f32 {
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2 - 1] + arr[len / 2]) as f32 / 2.0
    } else {
        arr[len / 2] as f32
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter a sorted array of integers (space separated): ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let arr: Vec<i32> = input.trim().split_whitespace().map(|num| num.parse().unwrap()).collect();

    let med = median(&arr);
    println!("The median is: {}", med);
}
