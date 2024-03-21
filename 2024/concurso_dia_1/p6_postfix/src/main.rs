fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// RPN/Evaluacion Postfija
// 9 8 7 + * 10 - 2 ^ 5

// 3125

fn main() {
    // Se obtiene los datos de entrada
    let input_str: Vec<String> = read_input()
        .split_whitespace()
        .map(std::string::ToString::to_string)
        .collect();

    // Si se regresa algun valor, se imprime, en caso de que se regrese None, se imprime que no es valido
    if let Some(number) = postfix(input_str) {
        println!("{number}");
    } else {
        println!("Not Valid");
    }
}

fn postfix(input_str: Vec<String>) -> Option<f32> {
    // Se inicializa la pila
    let mut stack = Vec::new();

    // Por cada numero o operador
    for x in input_str {
        // Si se puede convertir a numero flotante, es porque es un numero, no un operador
        if let Ok(num) = x.parse::<f32>() {
            // Se coloca en la pila
            stack.push(num);
        } else {
            // Si es un operador, se obtienen los 2 ultimos valores
            // En caso de que no existan los valores, se regresa None
            let n2 = stack.pop()?;
            let n1 = stack.pop()?;

            // Se busca que operador es, y se realiza la operacion, en caso de que no sea ninguno, se regresa None
            let res = match x.as_str() {
                "+" => n1 + n2,
                "-" => n1 - n2,
                "*" => n1 * n2,
                "/" => n1 / n2,
                "^" => n1.powf(n2),
                _ => return None,
            };

            // Se vuelve a guardar el resultado en la pila
            stack.push(res);
        }
    }

    // Se obtiene el ultimo valor, en caso de que no este vacia la pila, se regresa None
    let result = stack.pop();
    if stack.is_empty() {
        result
    } else {
        None
    }
}
