fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let num: i32 = read_input().trim().parse().unwrap();

    let mut guess = 0;
    let mut tries = 1;
    let mut top = 100;
    let mut lower = 0;
    let mut input;

    while num != guess {
        guess = (top + lower) / 2;
        println!("Intento {tries}: {guess}");

        loop {
            input = read_input().trim().parse().unwrap();
            match input {
                '<' | '>' | '=' => break,
                _ => {}
            }
        }

        match input {
            '>' => lower = guess,
            '<' => top = guess,
            '=' => {
                break;
            }
            _ => {}
        }
        tries += 1;

        if top < num || lower > num {
            println!("Not Posible");
            panic!("HELPPPP!");
        }
    }
    println!("Adivine en {tries} intentos");
}
