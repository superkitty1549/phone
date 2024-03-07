struct Nim {
    piles: Vec<usize>,
    num: usize,
    default: Vec<usize>
}

impl Nim {
    fn new(num: usize, default: Vec<usize>) -> Self {
        Self {
            piles: default.clone(),
            num: num,
            default: default,
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
            println!("u cant do dat");
            return 1;
        }
        if amt > self.piles[idx] {
            println!("u cant do dat");
            return 1;
        }
        self.piles[idx] -= amt;
        return 0;
    }
}

fn main() {
    let num_piles: usize = get_input("Number of piles: ")
        .parse::<usize>()
        .unwrap_or(3);

    let pile_sizes: Vec<usize> = get_input("Size of each pile (a b c): ")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap_or(10));
        .collect::<Vec<usize>>();

    let game: Nim = Nim::new(num_piles, pile_sizes);
    loop {
        
        if game.piles == vec![0; num] {
            println!("U win!");
            return 0;
        }
    }

}

fn 2p(game: &mut Game) {

}

fn 1p(game: &mut Game) {
    
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
