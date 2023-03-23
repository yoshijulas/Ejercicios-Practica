use std::fs;

fn main() -> std::io::Result<()> {
    let data = fs::read_to_string("input.txt")?;
    let mut lines = data.lines();
    let n: u32 = lines.next().unwrap().parse().unwrap();
    let k: u32 = lines.next().unwrap().parse().unwrap();
    println!("n: {n}, k: {k}");
    Ok(())
}
