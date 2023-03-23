#![feature(int_roundings)]

fn main() {
    let vec: Vec<i32> = (0..50).rev().collect();

    let sorted = merge_sort(vec.clone());
    println!("{}", verify_sorted(&vec));
    println!("{}", verify_sorted(&sorted));
}

fn verify_sorted(list: &[i32]) -> bool {
    let n = list.len();

    if n == 0 || n == 1 {
        return true;
    }

    list[0] < list[1] && verify_sorted(&list[1..])
}

fn merge_sort(list: Vec<i32>) -> Vec<i32> {
    if list.len() == 1 {
        return list;
    }

    let (lh, rh) = split(&list);
    let left = merge_sort(lh);
    let right = merge_sort(rh);

    merge(&left, &right)
}

fn split(list: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mid = list.len().div_floor(2);
    (list[..mid].to_vec(), list[mid..].to_vec())
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut l: Vec<i32> = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            l.push(left[i]);
            i += 1;
        } else {
            l.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        l.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        l.push(right[j]);
        j += 1;
    }

    l
}