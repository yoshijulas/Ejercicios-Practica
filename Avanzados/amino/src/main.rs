fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let length = read_input()
        .trim()
        .chars()
        .filter(|x| ['A', 'G', 'C', 'T'].contains(x))
        .count();

    if length % 3 == 0 {
        println!("{} {}", length / 3, length % 3);
    } else {
        println!("{} {}", length / 3, 3 - length % 3);
    }
}
