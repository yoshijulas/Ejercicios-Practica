fn main() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = linear_search(list.clone(), 12);
    verify(result);
    let result = linear_search(list, 6);
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

fn linear_search(list: Vec<i32>, target: i32) -> Option<i32> {
    for i in list {
        if i == target {
            return Some(i - 1);
        }
    }
    None
}
