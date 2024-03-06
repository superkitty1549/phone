fn main() {
    println!("CALC V.0.0.1\n'q' to quit\n");
    loop {
        let input: String = get_input(">> ");
        if input.get(0).unwrap_or(' ') == 'q' {
            break;
        }

    }
}

fn evaluate(expr: &str) -> Option<i32> {
    todo!();
}

fn get_input(message: &str) -> String {
    print!("{}", message);
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let mut input: String = String::new();
    match std::io::stdin()
        .read_line(&mut input) {
            Ok(_) => return String::from(input
                .trim()),
            Err(_) => return String::from("Unable to read input")
        }
}