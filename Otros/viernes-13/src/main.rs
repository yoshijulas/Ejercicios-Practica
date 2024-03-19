fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let year = 2000;
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    let day_index = starting_day(year);
    println!(
        "The year {} starts on a {}, {day_index}",
        year, days[day_index as usize]
    );
}

fn is_friday_13(year: i32) -> bool {
    let leap = leap_year(year);
    let mut year = 0;

    // 3 - Miercoles
    starting_day(year);

    match (year, leap) {
        (1, _) => {
            year += 1;
        }
        (2, true) => {
            year += 1;
        }
        (2, false) => {
            year += 1;
        }
        (3, _) => {
            year += 1;
        }
        (4, _) => {
            year += 1;
        }
        (5, _) => {
            year += 1;
        }
        (6, _) => {
            year += 1;
        }
        (7, _) => {
            year += 1;
        }
        (8, _) => {
            year += 1;
        }
        (9, _) => {
            year += 1;
        }
        (10, _) => {
            year += 1;
        }
        (11, _) => {
            year += 1;
        }
        (12, _) => {
            year += 1;
        }
        (_, _) => {}
    }

    true
}

fn starting_day(year: i32) -> i32 {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = if year < 3 { year - 1 } else { year };
    (year + year / 4 - year / 100 + year / 400 + t[0] + 1) % 7
}

fn leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
