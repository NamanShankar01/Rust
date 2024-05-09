use std::io::{self, Write};

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_so_far = arr[0];
    let mut max_ending_here = arr[0];

    for &num in &arr[1..] {
        max_ending_here = i32::max(num, max_ending_here + num);
        max_so_far = i32::max(max_so_far, max_ending_here);
    }

    max_so_far
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

    let max_sum = max_subarray_sum(&arr);
    println!("The maximum subarray sum is: {}", max_sum);
}
