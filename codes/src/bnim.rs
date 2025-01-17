use rand::Rng;
use std::io;


fn main() {
    let mut rng = rand::thread_rng();


    loop {
        play_game(&mut rng);


        println!("or... wanna play again? [y][n] ");
        let play_again = read_input().trim().to_lowercase();
        if play_again != "y" {
            break;
        }
    }
}


fn play_game(rng: &mut impl Rng) {
    // Get the number of piles from the player
    let num_piles: usize;
    loop {
        println!("how many piles do u want?");
        let input = read_input();
        if let Ok(value) = input.parse() {
            num_piles = value;
            break;
        } else {
            println!("no, enter a valid number");
        }
    }


    // Generate a random number of objects for each pile
    let mut piles: Vec<u32> = (0..num_piles).map(|_| rng.gen_range(0..=20)).collect();


    // Game loop
    while !piles.iter().all(|&x| x == 0) {
        // Display the current state of piles
        println!("current position: {:?}", piles);


        // Player's move
        println!("ur turn, or quit if ur a coward");
        let player_input = read_input();
        if player_input.trim().to_lowercase() == "quit" {
            println!("wow u actually resigned... l bozo");
            break;
        }
        let player_move = parse_move(&player_input, &piles);
        if player_move.is_empty() {
            println!("no stupid, enter a valid move (separate numbers with commas):");
            continue;
        }
        let mut updated_piles = piles.clone();
        update_piles(&mut updated_piles, &player_move);


        // Ensure only one pile has been changed
        let changes = piles.iter().zip(updated_piles.iter()).filter(|&(a, b)| a != b).count();
        if changes != 1 {
            println!("You can only change one pile at a time.");
            continue;
        }


        println!("ur move: {:?}", updated_piles);


        // Check if the game is won after player's move
        if updated_piles.iter().all(|&x| x == 0) {
            println!("wow good job, do u want a cookie or something?");
            break;
        }


        // Computer's move
        let computer_move = compute_move(&updated_piles, rng);
        update_piles(&mut piles, &computer_move);
        println!("computer's move: {:?}", piles);


        // Check if the game is won after computer's move
        if piles.iter().all(|&x| x == 0) {
            println!("womp womp idk go cry or smth lol");
            break;
        }
    }
}


// Function to read input from user
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}


// Function to parse player's move
fn parse_move(input: &str, _piles: &[u32]) -> Vec<u32> {
    if input.trim().to_lowercase() == "quit" {
        return vec![];
    }


    input
        .split(',')
        .map(|s| s.trim().parse().unwrap_or(0))
        .collect()
}


// Function to update the state of piles based on the move
fn update_piles(piles: &mut Vec<u32>, movement: &[u32]) {
    for (i, &num_objects) in movement.iter().enumerate() {
        piles[i] = num_objects;
    }
}


fn compute_move(piles: &[u32], _rng: &mut impl Rng) -> Vec<u32> {
    let mut piles = piles.to_vec();
    let nim_sum = piles.iter().fold(0, |acc, &pile| acc ^ pile);


    if nim_sum == 0 {
        // Halve the largest pile if nim sum is zero
        if let Some((index, _)) = piles.iter().enumerate().max_by_key(|&(_, &val)| val) {
            piles[index] /= 2;
        }
    } else {
        // Modify a pile to make the nim sum zero
        for i in 0..piles.len() {
            let potential_nim_sum = nim_sum ^ piles[i];
            // Find a pile that can be modified to make the nim sum zero
            if potential_nim_sum < piles[i] {
                piles[i] = potential_nim_sum;
                break;
            }
        }
    }


    piles
}
