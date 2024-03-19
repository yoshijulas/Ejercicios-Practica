fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Matriz de Identidad
// 1 0 0
// 0 1 0
// 0 0 1

// Sí

fn main() {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    loop {
        let input_line: Vec<i32> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if input_line.is_empty() {
            break;
        }

        grid.push(input_line);
    }

    if is_identity(&grid) {
        println!("Si");
    } else {
        println!("No");
    }
}

fn is_identity(grid: &[Vec<i32>]) -> bool {
    for (i, values) in grid.iter().enumerate() {
        for (j, &val) in values.iter().enumerate() {
            if i == j {
                if val != 1 {
                    return false;
                }
            } else if val != 0 {
                return false;
            }
        }
    }
    true
}
