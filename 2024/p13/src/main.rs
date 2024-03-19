fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Comprobar Número Perfecto
// 28

fn main() {
    let number: usize = read_input().trim().parse().unwrap();

    if perfect_number(number) {
        println!("Si");
    } else {
        println!("No");
    }
}

fn perfect_number(n: usize) -> bool {
    let mut sum = 0;

    for num in 1..n {
        if n % num == 0 {
            sum += num;
        }
    }
    sum == n
}
