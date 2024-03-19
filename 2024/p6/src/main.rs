use std::collections::VecDeque;

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Convertir a Binario

fn main() {
    let num: usize = read_input().trim().parse().unwrap();

    let binary = decimal_to_binary(num);
    for bin in binary {
        print!("{bin}");
    }
}

fn decimal_to_binary(num: usize) -> Vec<u8> {
    let mut sum = num;
    let mut binary: VecDeque<u8> = VecDeque::new();

    while sum > 0 {
        binary.push_front((sum % 2) as u8);
        sum /= 2;
    }

    binary.into_iter().collect::<Vec<u8>>()
}
