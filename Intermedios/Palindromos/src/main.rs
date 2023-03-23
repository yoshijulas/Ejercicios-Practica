use std::io::{self, Write};

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    print!("Ingrese un numero: ");
    io::stdout().flush().unwrap();
    let num = read_input().trim().to_string();

    let inv_num: Vec<char> = num.chars().rev().collect();

    if inv_num == num.chars().collect::<Vec<char>>() {
        println!("{num} es palindromo");
    } else {
        println!("{num} no es palindromo");
    }
}
