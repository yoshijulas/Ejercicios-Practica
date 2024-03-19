fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Calculadora de Suma de Subconjuntos
// 1 2 3 4 5
// 7

// 3

fn main() {
    // 1 2 3 4 5
    let input_vec: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let objetive = read_input().trim().parse::<i32>().unwrap();

    let index = 0;
    let suma = 0;
    find_subset(index, &mut Vec::new(), suma, &input_vec, objetive);

    //7
}
fn find_subset(index: usize, new_vec: &mut Vec<i32>, suma: i32, input_vec: &[i32], objetive: i32) {
    let n = input_vec.len();
    if index == n {
        if suma == objetive {
            print!("{new_vec:?}");
        }
        return;
    }

    // Pick
    new_vec.push(input_vec[index]);
    find_subset(
        index + 1,
        new_vec,
        suma + input_vec[index],
        input_vec,
        objetive,
    );

    // No Pick
    new_vec.pop();
    find_subset(index + 1, new_vec, suma, input_vec, objetive);
}
