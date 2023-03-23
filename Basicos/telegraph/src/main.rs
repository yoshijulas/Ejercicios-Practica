use std::io::{self, Write};

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    print!("Mensaje: ");
    io::stdout().flush().unwrap();
    let input: Vec<_> = read_input().trim().chars().map(|c| c as u8).collect();

    let amount = input
        .iter()
        .map(|ch| match ch {
            b'a'..=b'z' | b'A'..=b'Z' => 10,
            0..=9 => 20,
            b' ' => 0,
            _ => 30,
        })
        // .for_each(|ch| print!("{ch}"));
        .sum::<i32>();

    println!("Su mensaje cuesta {amount:?}");
}
