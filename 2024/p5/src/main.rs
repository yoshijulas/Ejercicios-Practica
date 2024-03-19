fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Buscar Elemento Mínimo
// 8 3 11 5 9

// 3

fn main() {
    let numbers: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut min = numbers[0];

    if numbers.len() > 1 {
        for i in numbers {
            if i < min {
                min = i;
            }
        }
    }

    println!("{min}");
}
