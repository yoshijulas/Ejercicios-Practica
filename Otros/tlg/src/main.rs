fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let n = read_input().trim().parse().unwrap();
    let mut max = 0;
    let mut winner = 0;

    for _ in 0..n {
        let line: Vec<i32> = read_input()
            .split_whitespace()
            .map(|ch| ch.parse().unwrap())
            .collect();

        let (a, b) = (line[0], line[1]);

        let c_delta = (a - b).abs();

        if c_delta > max {
            winner = i32::from(a > b);
            max = c_delta;
        }
    }

    println!("{winner} {max}");
}
