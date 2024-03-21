fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// El Primer Decimal Significativo
// 4
// 3.40 5.06 7.0800 0.50

// 23

fn main() {
    // Ignora el primer numero
    let _: usize = read_input().trim().parse().unwrap();
    // Obtiene los numeros
    let numbers: Vec<String> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Inicializa la suma en 0
    let mut sum = 0;

    // Suma todos los decimales
    for number in numbers {
        sum += sum_decimal(&number);
    }

    // Imprime la suma
    println!("{sum}");
}

fn sum_decimal(number: &str) -> usize {
    // Separa el numero por caracteres
    let ind = number.chars();
    // Si aun no se a encontrado el punto
    let mut dot = false;
    // Un for por cada uno de los caracteres
    for j in ind {
        // Si aun no se a encontrado el punto, y la posicion actual es el punto, coloca punto como verdadero
        if j == '.' && !dot {
            dot = true;
        }
        // Si ya se encontro el punto, y la posicion no es un 0, entonces, regresa el digito que encontro
        if dot && j != '0' && j != '.' {
            return j.to_digit(10).unwrap() as usize;
        }
    }
    // En caso de que no encuentre ningun punto, o numero que no sea 0, se regresa 0
    '0'.to_digit(10).unwrap() as usize
}
