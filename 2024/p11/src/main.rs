fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Suma de Números Primos
// 10

// 3 + 7 = 10
// 5 + 5 = 10
// 7 + 3 = 10

fn main() {
    let num: usize = read_input().trim().parse().unwrap();

    let posibles_num = sieve_of_eratosthenes(num);
    // let posibles_num = gen_prime_num(num);

    let correct_nums = suma_dos_numeros(&posibles_num, num);

    for i in correct_nums {
        println!("{} + {} = {num}", i.0, i.1);
    }
}

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut primes = vec![true; n + 1];
    primes[0] = false;
    primes[1] = false;

    for i in 2..=n {
        if primes[i] {
            for j in (i * i..=n).step_by(i) {
                primes[j] = false;
            }
        }
    }

    let mut result = Vec::new();
    for (i, &is_prime) in primes.iter().enumerate() {
        if is_prime {
            result.push(i);
        }
    }

    result
}

fn suma_dos_numeros(posibles_num: &[usize], num: usize) -> Vec<(usize, usize)> {
    let mut correct_nums: Vec<(usize, usize)> = Vec::new();

    for &i in posibles_num {
        for &j in posibles_num {
            if i + j == num {
                correct_nums.push((i, j));
            }
        }
    }
    correct_nums
}

fn gen_prime_num(num: usize) -> Vec<usize> {
    let mut prime_num = Vec::new();

    for i in 2..=num {
        if is_prime(i) {
            prime_num.push(i);
        }
    }
    prime_num
}

fn is_prime(num: usize) -> bool {
    if num <= 1 {
        return false;
    }

    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}
