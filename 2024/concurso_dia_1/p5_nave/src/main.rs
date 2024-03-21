fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Alfa Centauri

fn main() {
    // Se obtiene la cantidad de lineas
    let n: usize = read_input().trim().parse().unwrap();

    // Se inician variables definidas en el problema
    let mut tiempo = 6666;
    let mut calentamiento = 0;
    let mut gusanos = 0;

    // Se obtiene cada linea
    for _ in 0..n {
        let line: Vec<usize> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let (obstaculo, cuadrante) = (line[0], line[1]);

        // Si el obstaculo..
        match obstaculo {
            // Es 1 a 3, se suma 1 al tiempo
            1..=3 => tiempo += 1,
            // Si es 4 a 6, se suma 5 al tiempo, y si la temperatura es critica, se suma 2,
            // y solo si es calentamiento, se guarda en la variable
            4..=6 => {
                tiempo += 5;
                if cuadrante & 1 != 0 {
                    let temp = obstaculo * 10;
                    if temp <= 40 || temp >= 60 {
                        tiempo += 2;
                    }
                    if temp >= 60 {
                        calentamiento += 1;
                    }
                }
            }
            // Si es de 7 a 9, y es un gusano, se resta 10 al tiempo, y se suma
            7..=9 => {
                tiempo -= 10;
                gusanos += 1;
            }
            _ => (),
        }
    }

    // Se imprimen resultados
    println!("Total de dias = {tiempo}");
    println!("Total de tuneles de gusano = {gusanos}");
    println!("Total de estabilizaciones = {calentamiento}");
}
