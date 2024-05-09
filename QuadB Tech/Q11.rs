use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    println!("Enter the first sorted array of integers (space separated): ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let arr1: Vec<i32> = input.trim().split_whitespace().map(|num| num.parse().unwrap()).collect();

    input.clear();
    println!("Enter the second sorted array of integers (space separated): ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let arr2: Vec<i32> = input.trim().split_whitespace().map(|num| num.parse().unwrap()).collect();

    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);
    println!("The merged array is: {:?}", merged);
}
