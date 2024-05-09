fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut res = None;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            res = Some(mid);
            right = mid - 1;
        } 
        else if arr[mid] < target {
            left = mid + 1;
        } 
        else {
            right = mid - 1;
        }
    }
    res
}

fn main() {
    let arr = [1, 2, 2, 3, 3, 3, 4, 5, 6];
    let target = 3;
    match first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} is not in the array", target),
    }
}
