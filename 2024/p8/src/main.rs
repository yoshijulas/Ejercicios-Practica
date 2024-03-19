fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Contar Vocales y Consonantes

fn main() {
    let input_str: Vec<char> = read_input()
        .chars()
        .filter(char::is_ascii_alphabetic)
        .map(|x| x.to_ascii_lowercase())
        .collect();

    let mut vocales = 0;
    let mut consonantes = 0;

    for i in input_str {
        match i {
            'a' | 'e' | 'i' | 'o' | 'u' => vocales += 1,
            _ => consonantes += 1,
        }
    }

    print!("Vocales: {vocales} Consonantes: {consonantes}");
}
