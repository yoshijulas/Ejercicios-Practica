use rand::Rng;

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Generador de Contraseñas

fn main() {
    let lenght: usize = read_input().trim().parse().unwrap();

    println!("{}", generar_contrasena(lenght));
}

fn generar_contrasena(longitud: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut contrasena = String::new();
    for _ in 0..longitud {
        let caracter = match rng.gen_range(0..3) {
            0 => rng.gen_range('a'..='z'),
            1 => rng.gen_range('A'..='Z'),
            2 => rng.gen_range('0'..='9'),
            _ => unreachable!(),
        };
        contrasena.push(caracter);
    }
    contrasena
}
