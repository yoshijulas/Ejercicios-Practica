fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let size_of_matrix: usize = read_input().trim().parse().unwrap();

    let matrix_1: Vec<Vec<u8>> = read_matrix(size_of_matrix);
    let matrix_2: Vec<Vec<u8>> = read_matrix(size_of_matrix);

    let transformations = apply_transformations(&matrix_1, size_of_matrix);
    let transformations2 = apply_transformations(&matrix_2, size_of_matrix);

    let mut max = 0;
    let mut min = size_of_matrix * size_of_matrix;

    for matrix_1 in &transformations {
        for matrix_2 in &transformations2 {
            let common = common(matrix_1, matrix_2, size_of_matrix);

            max = max.max(common);
            min = min.min(common);
        }
    }

    println!("{max} {min}");
}

fn read_matrix(size: usize) -> Vec<Vec<u8>> {
    let mut matrix = Vec::new();
    for _ in 0..size {
        let row: Vec<u8> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        matrix.push(row);
    }
    matrix
}

fn apply_transformations(matrix: &[Vec<u8>], size: usize) -> Vec<Vec<Vec<u8>>> {
    vec![
        matrix.to_vec(),
        rotate_90(matrix, size),
        rotate_180(matrix, size),
        rotate_270(matrix, size),
        mirror_cols(matrix, size),
        mirror_rows(matrix, size),
    ]
}

fn common(matrix_1: &[Vec<u8>], matrix_2: &[Vec<u8>], size: usize) -> usize {
    let mut ammount = 0;

    for i in 0..size {
        for j in 0..size {
            if matrix_1[i][j] == matrix_2[i][j] {
                ammount += 1;
            }
        }
    }

    ammount
}

fn rotate_90(matrix: &[Vec<u8>], size: usize) -> Vec<Vec<u8>> {
    let mut rotated_matrix = vec![vec![0; size]; size];

    for i in 0..size {
        for j in 0..size {
            rotated_matrix[j][size - i - 1] = matrix[i][j];
        }
    }

    rotated_matrix
}

fn rotate_180(matrix: &[Vec<u8>], size: usize) -> Vec<Vec<u8>> {
    let mut rotated_matrix = vec![vec![0; size]; size];

    for i in 0..size {
        for j in 0..size {
            rotated_matrix[size - i - 1][size - j - 1] = matrix[i][j];
        }
    }

    rotated_matrix
}

fn rotate_270(matrix: &[Vec<u8>], size: usize) -> Vec<Vec<u8>> {
    let mut rotated_matrix = vec![vec![0; size]; size];

    for i in 0..size {
        for j in 0..size {
            rotated_matrix[size - j - 1][i] = matrix[i][j];
        }
    }

    rotated_matrix
}

fn mirror_cols(matrix: &[Vec<u8>], size: usize) -> Vec<Vec<u8>> {
    let mut mirror_matrix = vec![vec![0; size]; size];

    for i in 0..size {
        for j in 0..size {
            mirror_matrix[size - i - 1][j] = matrix[i][j];
        }
    }

    mirror_matrix
}

fn mirror_rows(matrix: &[Vec<u8>], size: usize) -> Vec<Vec<u8>> {
    let mut mirror_matrix = vec![vec![0; size]; size];

    for i in 0..size {
        for j in 0..size {
            mirror_matrix[i][size - j - 1] = matrix[i][j];
        }
    }

    mirror_matrix
}
