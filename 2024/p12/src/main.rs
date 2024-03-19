fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Serie de fibonacci
// 8

// 0 1 1 2 3 5 8 13

fn main() {
    let n: usize = read_input().trim().parse().unwrap();

    let fib = fibonacci(n - 1);

    for num in fib {
        print!("{num} ");
    }
}

fn fibonacci(n: usize) -> Vec<i64> {
    let mut table: Vec<i64> = vec![0; n + 1];

    table[1] = 1;

    for i in 2..=n {
        table[i] = table[i - 1] + table[i - 2];
    }

    table
}
