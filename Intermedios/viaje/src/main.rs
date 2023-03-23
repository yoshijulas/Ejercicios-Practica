use std::io::{self, Write};

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let mut min = 0;
    loop {
        print!("Duracion Tramo: ");
        io::stdout().flush().unwrap();
        let input: i32 = read_input().trim().parse().unwrap();

        min += input;

        if input == 0 {
            break;
        }
    }

    println!(
        "Tiempo total del viaje: {}:{} horas",
        (min / 60),
        (min % 60)
    );
}
