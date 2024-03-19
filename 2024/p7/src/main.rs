fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Ordenar una Lista

fn main() {
    let mut numbers: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut changed = true;
    let mut n = numbers.len() - 1;

    while changed {
        changed = false;

        for i in 0..n {
            if numbers[i] > numbers[i + 1] {
                numbers.swap(i, i + 1);
                changed = true;
            }
        }
        n -= 1;
    }

    for num in numbers {
        print!("{num} ");
    }
}
