fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Gusano trepador
// 10 8 40

// 20

fn main() {
    let input: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (a, b, c) = (input[0], input[1], input[2]);

    println!("{}", (f64::from(c) / (f64::from(a) - f64::from(b))).ceil());
}
