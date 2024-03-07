use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut rng = rand::thread_rng();
    let commands = vec![
        "?roll d20",
        "?roll d6 3",
        "?flip",
        "?8ball",
        "?card",
    ];

    for command in commands {
        match command.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["?roll", dice] if dice.starts_with("d") => {
                let max: i32 = dice[1..].parse().unwrap_or(0);
                println!("{}", rng.gen_range(1..=max));
            }
            ["?roll", dice, times] if dice.starts_with("d") => {
                let max: i32 = dice[1..].parse().unwrap_or(0);
                let times: i32 = times.parse().unwrap_or(0);
                for _ in 0..times {
                    println!("{}", rng.gen_range(1..=max));
                }
            }
            ["?flip"] => {
                let flip = if rng.gen_bool(0.5) { "Heads" } else { "Tails" };
                println!("{}", flip);
            }
            ["?8ball"] => {
                let answers = vec![
                    "It is certain.",
                    "It is decidedly so.",
                    "Without a doubt.",
                    "Yes - definitely.",
                    "You may rely on it.",
                    "As I see it, yes.",
                    "Most likely.",
                    "Outlook good.",
                ];
                println!("{}", answers[rng.gen_range(0..answers.len())]);
            }
            ["?card"] => {
                let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
                let values = vec![
                    "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
                ];
                let suit = suits[rng.gen_range(0..suits.len())];
                let value = values[rng.gen_range(0..values.len())];
                println!("{} of {}", value, suit);
            }
            _ => println!("Unknown command"),
        }
    }
}
