fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Matriz Mágica
// 2 7 6
// 9 5 1
// 4 3 8

// Si

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

    if compare(&grid) {
        print!("Si");
    } else {
        print!("No");
    }
}

fn compare(grid: &[Vec<i32>]) -> bool {
    let n = grid.len();
    let mut row: Vec<i32> = vec![0; n];
    let mut col: Vec<i32> = vec![0; n];

    for i in 0..n {
        for j in 0..n {
            row[i] += grid[i][j];
            col[j] += grid[i][j];
        }
    }

    let mut diag1 = 0;
    let mut diag2 = 0;

    for i in 0..n {
        diag1 += grid[i][i];
        diag2 += grid[i][n - i - 1];
    }

    let magic_sum = row[0];

    for i in 0..n {
        if row[i] != magic_sum || col[i] != magic_sum {
            return false;
        }
    }

    if diag1 != magic_sum || diag2 != magic_sum {
        return false;
    }

    true
}
