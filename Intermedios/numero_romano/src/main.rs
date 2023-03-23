fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let input = read_input().trim().to_uppercase();
    let result = romano_a_arabigo(&input);
    println!("{result}");
}

fn romano_a_arabigo(num: &str) -> u32 {
    let input: Vec<u32> = num
        .chars()
        .map(|ch| match ch {
            'M' => 1000,
            'D' => 500,
            'C' => 100,
            'L' => 50,
            'X' => 10,
            'V' => 5,
            'I' => 1,
            _ => 0,
        })
        .collect();

    let mut total = 0;
    for (i, value) in input.iter().enumerate() {
        if i < input.len() - 1 && *value > input[i + 1] {
            total -= value;
        } else {
            total += value;
        }
    }

    total
}
