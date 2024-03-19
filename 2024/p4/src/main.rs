fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Verificar Palíndromo

fn main() {
    let input: String = read_input()
        .chars()
        .filter(|x| x.is_alphanumeric())
        .map(|x| x.to_ascii_lowercase())
        .collect();

    let mut is_palindrome = input == input.chars().rev().collect::<String>();

    let input_chars: Vec<char> = input.chars().collect();
    for i in 0..input_chars.len() / 2 {
        if input_chars[i] != input_chars[input_chars.len() - i - 1] {
            is_palindrome = false;
        }
    }

    if is_palindrome {
        println!("Si");
    } else {
        println!("No");
    }
}
