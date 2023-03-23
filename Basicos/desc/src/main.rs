use std::io::{self, Write};

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    print!("n: ");
    io::stdout().flush().unwrap();
    let n = read_input().trim().parse().unwrap();

    print!("Cantidad productos: ");
    io::stdout().flush().unwrap();
    let products = read_input().trim().parse().unwrap();

    let mut total = 0;
    let mut discount_total = 0;
    let mut final_price = 0;
    let mut ammount = 1;

    let mut discount = 20;

    for i in 1..=products {
        print!("Precio producto {i}: ");
        io::stdout().flush().unwrap();
        let value: i32 = read_input().trim().parse().unwrap();
        total += value;
        discount_total += (value * discount) / 100;
        final_price += value - ((value * discount) / 100);

        // DEBUG
        println!(
            "D:{discount}, A:{ammount}, VT:{value}, DA:{}, VF:{}",
            ((value * discount) / 100),
            (value - (value * discount) / 100)
        );

        if ammount == n {
            discount /= 2;
            ammount = 1;
        } else {
            ammount += 1;
        }
    }

    println!("Total: {total}");
    println!("Descuento: {discount_total}");
    println!("Por pagar: {final_price}");
}
