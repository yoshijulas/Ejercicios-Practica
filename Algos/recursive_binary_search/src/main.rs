#![feature(int_roundings)]

use std::cmp::Ordering;
fn main() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = recursive_binary_search(&list, 12);
    verify(result);
    let result = recursive_binary_search(&list, 6);
    verify(result);
}

fn verify(result: bool) {
    if result {
        println!("Founded");
    } else {
        println!("Not founded");
    }
}

fn recursive_binary_search(list: &[i32], target: i32) -> bool {
    if list.is_empty() {
        return false;
    }

    let midpoint = (list.len()).div_floor(2);

    match list[midpoint].cmp(&target) {
        Ordering::Equal => true,
        Ordering::Less => recursive_binary_search(&list[midpoint + 1..], target),
        Ordering::Greater => recursive_binary_search(&list[..midpoint], target),
    }
}
