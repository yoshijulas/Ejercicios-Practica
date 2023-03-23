use std::cmp::Ordering;

fn main() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = binary_search(&list, 12);
    verify(result);
    let result = binary_search(&list, 6);
    verify(result);
}

fn verify(result: Option<i32>) {
    result.map_or_else(
        || {
            println!("Not Found");
        },
        |value| {
            println!("The value is at: {value}");
        },
    );
}

fn binary_search(list: &[i32], target: i32) -> Option<i32> {
    let mut first = 0;
    let mut last = list.len() - 1;

    while first <= last {
        let midpoint: usize = ((first as f32 + last as f32) / 2.0).floor() as usize;

        match list[midpoint].cmp(&target) {
            Ordering::Equal => return Some(midpoint.try_into().unwrap()),
            Ordering::Greater => last = midpoint - 1,
            Ordering::Less => first = midpoint + 1,
        }
    }
    None
}
