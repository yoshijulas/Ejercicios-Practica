fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Cuadrado Latino
// 1 2 3
// 2 3 1
// 3 1 2

// Sí

fn main() {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    let mut num = 12;
    while num != 0 {
        let input_line: Vec<usize> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if num == 12 {
            num = input_line.len();
        }

        grid.push(input_line);
        num -= 1;
    }

    if is_valid_sudoku(&grid) {
        println!("Si");
    } else {
        println!("No");
    }
}

fn is_valid_sudoku(grid: &[Vec<usize>]) -> bool {
    let length = grid.len();
    let mut row_check = vec![false; length];
    let mut col_check = vec![false; length];

    for i in 0..length {
        for j in 0..length {
            let num_row = grid[i][j];
            if num_row > length {
                return false;
            }
            if row_check[num_row - 1] {
                return false;
            }
            row_check[num_row - 1] = true;

            let num_col = grid[j][i];
            if num_col > length {
                return false;
            }
            if col_check[num_col - 1] {
                return false;
            }
            col_check[num_col - 1] = true;
        }

        row_check = vec![false; length];
        col_check = vec![false; length];
    }

    true
}
