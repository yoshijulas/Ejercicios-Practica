fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let num = read_input().trim().parse().unwrap();
    iterative_number(num, num, 0);
}

fn normal(num: f64) -> f64 {
    num.powi(3) + 4.0 * num.powi(2) - 10.0
}

fn derivada(num: f64) -> f64 {
    4.0 * num.powi(2) + 8.0 * num
}

fn full(num: f64) -> (f64, f64, f64) {
    let normal = normal(num);
    let derivada = derivada(num);
    let new_num = num - (normal / derivada);

    (new_num, normal, derivada)
}

fn iterative_number(mut num_init: f64, mut last: f64, mut i: i32) -> (f64, i32) {
    loop {
        let (num_next, normal, derivada) = full(num_init);

        let diff = (num_init - last).abs();

        if diff < 0.000_000_000_1 && i != 0 {
            println!(
                "+---------------------------------------------------------------------------+"
            );
            return (num_next, i);
        }

        if i == 0 {
            println!(
                "+---------------------------------------------------------------------------+\n\
                | {:>3} | {:^11} | {:^11} | {:^11} | {:^11} | {:^11} |\n\
                +---------------------------------------------------------------------------+",
                "i", "xi", "f(xi)", "f'(xi)", "xi + 1", "xi_xi"
            );
        }

        println!(
            "| {i:3} | {num_init:11.8} | {normal:11.8} | {derivada:11.8} | {num_next:11.8} | {diff:11.8} |"
        );

        last = num_init;
        num_init = num_next;
        i += 1;
    }
}
