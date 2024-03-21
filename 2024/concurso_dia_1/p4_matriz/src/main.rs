fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Matriz de puro cero
// 3 3
// 1 1 1
// 1 0 1
// 1 1 1

// 1 0 1
// 0 0 0
// 1 0 1

fn main() {
    // Se obtienen n, y m
    let num: Vec<usize> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Se separa en sus propias variables
    let (n, m) = (num[0], num[1]);

    // Se crea la matriz
    let mut matriz = Vec::new();

    // Se obtienen todas las filas y se meten el la matriz
    for _ in 0..n {
        let line: Vec<usize> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        matriz.push(line);
    }

    // Imprime el resultado de la Matriz
    let finish_matriz = matriz_cero(n, m, matriz);

    println!();

    for line in finish_matriz {
        for k in line {
            print!("{k} ");
        }
        println!();
    }
}

fn matriz_cero(n: usize, m: usize, matriz: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    // Crea una matriz nueva con los mismos valores en la Matriz para poder ser modificada
    let mut new_matriz = matriz.clone();

    // Se busca en i, j
    for i in 0..n {
        for j in 0..m {
            // Si en la posicion actual hay un 0
            if matriz[i][j] == 0 {
                // Se cambia la fila y columna a 0
                for i_l in 0..n {
                    new_matriz[i_l][j] = 0;
                }
                for j_l in 0..m {
                    new_matriz[i][j_l] = 0;
                }
            }
        }
    }

    // Se regresa la matriz modificada
    new_matriz
}
