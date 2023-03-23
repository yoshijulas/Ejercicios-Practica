use std::vec;

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let _ = read_input();

    let number: Vec<i32> = read_input()
        .split_whitespace()
        .map(|ch| ch.parse().unwrap())
        .collect();

    let number_2 = number.clone();
    // println!("{number:?} {number_2:?}");

    let (counter_asc, vec_asc) = bubble_sort(number, true);
    let (counter_desc, vec_desc) = bubble_sort(number_2, false);

    print_vec(counter_asc, vec_asc);
    print_vec(counter_desc, vec_desc);
}

fn bubble_sort(mut number: Vec<i32>, ascending: bool) -> (i32, Vec<Vec<i32>>) {
    let mut vec_iters: Vec<Vec<i32>> = vec![vec![]];
    let mut changed = true;
    let mut counter = 0;

    let n = number.len() - 1;

    while changed {
        changed = false;
        for i in 0..n {
            if (ascending && number[i] > number[i + 1]) || (!ascending && number[i] < number[i + 1])
            {
                number.swap(i, i + 1);
                counter += 1;
                vec_iters.push(number.clone());
                changed = true;
            }
        }
        // n -= 1;
    }

    (counter, vec_iters)
}

fn print_vec(counter: i32, vec_out: Vec<Vec<i32>>) {
    println!();

    print!("{counter}");
    for i in vec_out {
        for j in i {
            print!("{j} ");
        }
        println!();
    }
}
