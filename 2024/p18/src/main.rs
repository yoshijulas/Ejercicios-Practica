fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Otro ladrillo en la pared
// 2 10 7
// 5 5 5 3 5 2 2

// NO

fn main() {
    let input_line: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (h, w, _n) = (input_line[0], input_line[1], input_line[2]);

    let numbers: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut sum = 0;
    let mut row = 0;

    for num in numbers {
        sum += num;
        if sum == w {
            row += 1;

            if row == h {
                println!("YES");
                break;
            }
            sum = 0;
        }

        if sum > w {
            println!("NO");
            break;
        }
    }
}
