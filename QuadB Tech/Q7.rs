use std::io::{self, Write};

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_unstable();
    sorted_arr.get(k - 1).copied()
    //Some(sorted_arr[k-1]) --> Index Out of Bound case causes Panic
}

fn main() {
    let mut input = String::new();
    println!("Enter an array of integers (space separated): ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let arr: Vec<i32> = input.trim().split_whitespace().map(|num| num.parse().unwrap()).collect();

    input.clear();
    println!("Enter the value of k: ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let k: usize = input.trim().parse().unwrap();

    match kth_smallest(&arr, k) {
        Some(val) => println!("The {}th smallest element is: {}", k, val),
        None => println!("The array does not have a {}th element", k),
    }
}
