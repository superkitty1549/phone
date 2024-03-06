fn main() {
    println!("\nCALC V.0.0.1\n'q' to quit\n");
    loop {
        let input: String = get_input(">> ");
        if &input == "q" {
            break;
        }
        evaluate(&input);
    }
    println!("");
}

fn evaluate(expr: &str) {
    if expr.starts_with("trig") {
        let arg = expr.get(5..).unwrap_or("").trim();
        match arg.parse::<i32>() {
            Ok(num) => {
                let calculated: f64 = trig(num as f64);
                println!("{} = {}", expr, calculated);
            }
            Err(_) => {
                println!("Argument to trig function must be an integer: {}", arg);
            }
        }
    } else if expr.starts_with("d/dx") {
        if expr.len() > 5 {
            let result = differentiate(&expr[5..]);
            println!("{} = {}", expr, result);
        } else {
            println!("Error: Expression too short for differentiation");
        }
    } else {
        println!("Invalid expression")
    }
}

fn trig(num: f64) -> f64 {
    match rand(0, 2) {
        0 => num.sin(),
        1 => num.cos(),
        2 => num.tan(),
        _ => unreachable!()
    }
}

fn rand(lower: i32, upper: i32) -> i32 {
    if upper <= lower {
        return upper;
    }
    ((((&vec![2, 3]) as *const Vec<i32>) as i32) % (upper - lower) + lower).abs()
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

fn differentiate(expr: &str) -> String {
    let mut expr = expr.replace(" ", "");
    expr.push('+');
    let mut iter = expr.chars().peekable();
    let mut result = String::new();
    result.push_str(&simplify_expression(&differentiate_helper(&mut iter)));
    result
}

fn differentiate_helper(iter: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut result = String::new();
    let mut current_term = String::new();
    let mut operator = '+';

    while let Some(c) = iter.next() {
        match c {
            '+' | '-' => {
                let term_derivative = differentiate_term(&current_term);
                if operator == '-' {
                    result.push_str(&format!("-{}", term_derivative));
                } else {
                    result.push_str(&term_derivative);
                }
                operator = c;
                current_term.clear();
            }
            _ => current_term.push(c),
        }
    }

    result
}

fn differentiate_term(term: &str) -> String {
    if term.contains("+") || term.contains("-") {
        return differentiate(&term);
    }

    if term.contains("*") {
        let mut iter = term.split("*");
        let left = iter.next().unwrap();
        let right = iter.next().unwrap();
        return format!(
            "({})*({}) + ({}) * ({})",
            differentiate(left),
            right,
            left,
            differentiate(right)
        );
    }

    if term.contains("/") {
        let mut iter = term.split("/");
        let left = iter.next().unwrap();
        let right = iter.next().unwrap();
        return format!(
            "(({}) * ({})) - (({}) * ({})) / (({})^2)",
            differentiate(left),
            right,
            left,
            differentiate(right),
            right
        );
    }

    if term.contains("^") {
        let mut iter = term.split("^");
        let base = iter.next().unwrap();
        let exponent = iter.next().unwrap();
        if let Ok(_) = base.parse::<f64>() {
            return format!("{}^{} * ln({})", base, exponent, base);
        } else {
            return format!(
                "({})*({}^({}-1))",
                exponent,
                base,
                exponent
            );
        }
    }

    if let Ok(_) = term.parse::<f64>() {
        return "0".to_string();
    }

    if term == "x" {
        return "1".to_string();
    }

    if term.starts_with("x^") {
        return term.replace("x", "").replace("^", "");
    }

    term.to_string()
}

fn simplify_expression(expr: &str) -> String {
    let mut result = String::new();
    let mut terms = expr.split('+').collect::<Vec<&str>>();
    terms.sort();
    for term in terms {
        if term.starts_with('-') {
            result.push_str(&term);
        } else {
            result.push('+');
            result.push_str(&term);
        }
    }
    if result.starts_with('+') {
        result.remove(0);
    }
    result
}
