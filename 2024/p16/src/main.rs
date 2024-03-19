fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Contador de números en matriz

fn main() {
    let input: Vec<usize> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let numbers: Vec<usize> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut cantidad: Vec<usize> = vec![0; input[1]];

    for num in numbers {
        cantidad[num - 1] += 1;
    }

    for (idx, values) in cantidad.into_iter().enumerate() {
        println!("{}: {values} veces", idx + 1);
    }
}
