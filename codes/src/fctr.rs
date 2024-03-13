fn factors(n: u32) -> Vec<u32> {
    let limit = (n as f64).sqrt() as u32;
    let mut factors: Vec<u32> = Vec::new();

    let mut remaining = n;
    let current_factor = 2;

    while remaining % current_factor == 0 {
        factors.push(current_factor);
        remaining /= current_factor;
    }
    
    for i in (3..=limit).step_by(2) {
        while remaining % i == 0 {
            factors.push(i);
            remaining /= i;
        }
    }

    if remaining > 2 {
        factors.push(remaining);
    }

    factors
}

fn input() -> u32 {
    print!(">> ");
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let mut num: String = String::new();
    let _ = std::io::stdin().read_line(&mut num);
    match num.trim().parse::<u32>() {
        Ok(val) => val,
        Err(_) => {
            print!("Invalid input");
            input()
        }
    }
}

fn main() {
    println!("FCTR");
    let n: u32 = input();
    println!("prime factors of {}: {:?}", n, factors(n));
}
