use std::io;

fn main() {
    let pi_digits = "314159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706798214808651328230664709384460955058223172535940812848111745028410270193852110555964462294895493038196442881097566593344612847564823378678316527120190914564856692346034861045432664821339360682019";
    let mut user_input = String::new();
    let mut current_index = 0;

    println!("Welcome to the Pi game!");
    println!("Start by typing as many digits of Pi as you can remember. When you're ready to guess the next digit, just press enter.");

    loop {
        user_input.clear();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input = user_input.trim();

        if user_input.len() > current_index {
            let user_attempt = &user_input[current_index..];
            if pi_digits.starts_with(user_input) {
                println!("Correct so far! What's the next digit of Pi?");
                current_index = user_input.len();
            } else {
                println!("Oops! That's not right. The next digit was '{}'. Let's start over!", pi_digits.chars().nth(current_index).unwrap());
                current_index = 0;
            }
        } else if user_input.is_empty() {
            let next_digit = pi_digits.chars().nth(current_index).unwrap();
            println!("What's the next digit of Pi?");
            user_input.clear();
            io::stdin().read_line(&mut user_input).expect("Failed to read line");
            let guess = user_input.trim();

            if guess == next_digit.to_string() {
                println!("Correct! Keep going.");
                current_index += 1;
            } else {
                println!("Incorrect. The next digit was '{}'. Try again from the beginning!", next_digit);
                current_index = 0;
            }
        }

        if current_index >= 500 {
            println!("Congratulations! You've reached the end of the game.");
            break;
        }
    }
}
