use std::io::{self, Write};

fn read_input(calif: &str) -> f32 {
    let mut input_str = String::new();
    print!("{calif}: ");
    io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str.trim().parse().unwrap()
}

fn main() {
    let c1: f32 = read_input("C1");
    let c2: f32 = read_input("C2");

    if (c1 <= 1.9) || (c2 <= 1.9) {
        println!("Reprobado");
        return;
    }

    if (c1 >= 9.0) || (c2 >= 9.0) {
        println!("Aprovado");
        return;
    }

    let c3: f32 = read_input("C3");
    let prom = (c1 + c2 + c3) / 3.0;

    if prom <= 3.0 {
        println!("Reprobado");
        return;
    }

    if prom >= 7.0 {
        println!("Aprovado");
        return;
    }

    let exam: f32 = read_input("Examen");
    if exam >= 5.0 {
        println!("Aprovado");
    } else {
        println!("Reprobado");
    }
}

// match (c1, c2) {
//     (..=1.9, ..=1.9) => println!("Reprobado"),
//     (9.0.., 9.0..) => println!("Aprovado"),
//     (_, _) => {
//         let c3: f32 = read_input("C3").trim().parse().unwrap();
//         prom = (c1 + c2 + c3) / 3.0;
//         match prom {
//             ..=3.0 => println!("Reprobado"),
//             7.0.. => println!("Aprovado"),
//             _ => {
//                 let exam: f32 = read_input("Examen").trim().parse().unwrap();
//                 if exam >= 5.0 {
//                     println!("Aprovado");
//                 } else {
//                     println!("Reprobado");
//                 }
//             }
//         }
//     }
// }
