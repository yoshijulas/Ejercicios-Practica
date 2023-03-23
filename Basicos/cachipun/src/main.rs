use std::io;
use std::io::Write;

fn read_input(player: &str) -> String {
    loop {
        print!("{player}: ");
        io::stdout().flush().unwrap();
        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");
        let input_str = input_str.trim().to_lowercase();
        if ["piedra", "papel", "tijera"].contains(&input_str.as_str()) {
            return input_str;
        }
    }
}
fn main() {
    let mut a = 0u8;
    let mut b = 0u8;

    loop {
        match (read_input("A").as_str(), read_input("B").as_str()) {
            ("tijera", "papel") | ("piedra", "tijera") | ("papel", "piedra") => a += 1,
            ("papel", "tijera") | ("tijera", "piedra") | ("piedra", "papel") => b += 1,
            (_, _) => continue,
        }

        println!("{a} - {b}");

        if a == 3 || b == 3 {
            break;
        }
    }

    if a > b {
        println!("A es ganador");
    } else {
        println!("B es ganador");
    }
}
