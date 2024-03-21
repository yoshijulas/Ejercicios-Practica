fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Suma progresion aritmetica
// 3 2 5

// 35

fn main() {
    // Lee los numeros y los separa por espacios, y los une en un vector de usize
    let numbers: Vec<usize> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Inicia la suma
    let mut sum = 0;

    // Crea un for para la cantidad de numeros a sumar
    for i in 0..numbers[2] {
        // Suma el primer numero, mas las veces que se va a sumar el diferencia
        sum += numbers[0] + (numbers[1] * i);
    }

    // Se inprime el resultado
    println!("{sum}");
}
