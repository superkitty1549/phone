struct Nim {
    piles: Vec<usize>,
    num: usize,
    default: Vec<usize>,
}

impl Nim {
    fn new(num: usize, default: Vec<usize>) -> Self {
        Self {
            piles: default.clone(),
            num,
            default,
        }
    }

    fn sum(&self) -> usize {
        self.piles.iter().fold(0, |acc, &pile| acc ^ pile)
    }

    fn optimal_move(&mut self) {
        let nim_sum_value = self.sum();

        if nim_sum_value == 0 {
            if let Some(max_pile_index) = self.piles.iter().enumerate().max_by_key(|&(_, &x)| x) {
                let pile_index = max_pile_index.0;
                self.piles[pile_index] /= 2;
            }
        } else if let Some((pile_index, pile)) = self.piles.iter_mut().enumerate().find(|&(_, &mut pile)| pile ^ nim_sum_value < pile) {
            *pile ^= nim_sum_value;
        }
    }

    fn apply_move(&mut self, idx: usize, amt: usize) -> bool {
        if idx >= self.num {
            println!("Invalid pile index");
            return true;
        }
        if amt > self.piles[idx] {
            println!("Invalid move: Not enough stones in the pile");
            return true;
        }
        self.piles[idx] -= amt;
        false
    }

    fn display(&self) {
        println!("{:?}", self.piles);
    }

    fn accept_move(&mut self, player: &str) {
        let message: String = format!("{player}: ");
        let input = get_input(&message);
        let mut pile_values: Vec<usize> = Vec::new();
    
        for x in input.split_whitespace() {
            let pile_value = x.parse::<usize>().unwrap_or(0);
            pile_values.push(pile_value);
        }
    
        if pile_values.len() != self.num {
            println!("Invalid move: Please specify the new values for all piles.");
            self.accept_move(player);
            return;
        }
    
        for i in 0..self.num {
            if pile_values[i] > self.piles[i] {
                println!("Invalid move: New pile value must be less than or equal to the original value.");
                self.accept_move(player);
                return;
            }
        }
        
        let mut changed = 0;
        for (idx, i) in pile_values.iter().enumerate() {
            if *i as i32 - self.piles[idx] as i32 != 0 {
                changed += 1;
            }
        }
        
        if changed == 0 {
            println!("Invalid move: Must change one pile");
            self.accept_move(player);
            return;
        }
        
        if changed > 1 {
            println!("Invalid move: can only change one pile");
            self.accept_move(player);
            return;
        }
    
        self.piles = pile_values;
    }

    fn sp(&mut self) {
        let mut players_turn: bool = false;
        while self.piles.iter().sum::<usize>() > 0 {
            self.accept_move("Player");
            self.display();
            players_turn = !players_turn;
            if self.piles.iter().sum::<usize>() == 0 {
                break;
            }
            println!("Computer: ");
            self.optimal_move();
            self.display();
            players_turn = !players_turn;
        }
        if players_turn {
            println!("You win!");
        } else {
            println!("You lose!");
        }
    }

    fn mp(&mut self) {
        let mut player = 1;
        while self.piles.iter().sum::<usize>() > 0 {
            self.display();
            let player_name = if player == 1 { "Player 1" } else { "Player 2" };
            self.accept_move(player_name);
            if self.piles.iter().sum::<usize>() == 0 {
                println!("{} wins!", player_name);
                break;
            }
            player = if player == 1 { 2 } else { 1 };
        }
    }
}

fn main() {
    println!("nimx V.0.1.4");
    println!("'q' to quit\n");
    
    let num_piles: usize = get_input("Number of piles: ")
        .parse::<usize>()
        .unwrap_or(3);

    let mut pile_sizes: Vec<usize> = get_input("Size of each pile (space separated): ")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap_or(7))
        .collect::<Vec<usize>>();
    while pile_sizes.len() != num_piles {
        println!("Invalid starting position");
        pile_sizes = get_input("Size of each pile (space separated): ")
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap_or(7))
            .collect::<Vec<usize>>();
    }
    let multiplayer: bool = get_input("multiplayer? (y/n): ") == String::from("y");
    
    let mut game: Nim = Nim::new(num_piles, pile_sizes);
    if multiplayer {
        game.mp();
    } else {
        game.sp();
    }
}

fn get_input(message: &str) -> String {
    print!("{}", message);
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let mut input: String = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => return String::from(input.trim()),
        Err(_) => panic!("Unable to read input"),
    }
}