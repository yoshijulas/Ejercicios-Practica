fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Calculadora de Factorial
// 5

// 120

fn main() {
    let num = read_input().trim().parse().unwrap();

    println!("{}", factorial(num));
}

fn factorial(n: usize) -> usize {
    if n == 0 || n == 1 {
        return 1;
    }

    factorial(n - 1) * n
}
