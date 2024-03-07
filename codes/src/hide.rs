use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut hidden_apps: HashMap<String, String> = HashMap::new();
    let available_apps = vec![
        ("Calculator", "calc"),
        ("Notes", "note"),
        ("Email", "emai"),
        ("Browser", "brow"),
    ];
    let mut app_shorthands: HashMap<String, String> = HashMap::new();
    for app in &available_apps {
        app_shorthands.insert(app.1.to_string(), app.0.to_string());
    }

    loop {
        println!("Calculator App");
        println!("Enter operation or special code:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "3497937412" {
            println!("Special mode activated.");
            loop {
                println!("Available apps to hide:");
                for app in &available_apps {
                    println!("{} ({})", app.0, app.1);
                }
                println!("Hidden apps:");
                for (key, value) in &hidden_apps {
                    println!("{} ({})", value, key);
                }
                println!("Enter 'add {shorthand}' to hide an app, 'open {shorthand}' to open an app, or 'exit' to exit:");
                let mut command = String::new();
                io::stdin().read_line(&mut command).unwrap();
                let parts: Vec<&str> = command.trim().split_whitespace().collect();
                if parts.len() == 2 {
                    match parts[0] {
                        "add" => {
                            if let Some(app_name) = app_shorthands.get(parts[1]) {
                                hidden_apps.insert(parts[1].to_string(), app_name.clone());
                                println!("{} added to hidden apps.", app_name);
                            } else {
                                println!("Error: App not available.");
                            }
                        }
                        "open" => {
                            if let Some(app_name) = hidden_apps.get(parts[1]) {
                                println!("Opening {}...", app_name);
                            } else {
                                println!("Error: App not hidden or doesn't exist.");
                            }
                        }
                        _ => println!("Invalid command."),
                    }
                } else if parts.len() == 1 && parts[0] == "exit" {
                    break;
                } else {
                    println!("Invalid command.");
                }
            }
        } else {
            match input.parse::<f64>() {
                Ok(_) => println!("Result: {}", input),
                Err(_) => println!("Invalid input."),
            }
        }
    }
}
