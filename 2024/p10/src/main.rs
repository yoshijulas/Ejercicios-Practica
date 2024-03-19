fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Contar Palabras en un Texto

fn main() {
    let binding = read_input();

    println!("{}", binding.split_whitespace().count());
}
