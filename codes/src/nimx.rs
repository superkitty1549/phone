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

    fn display(&self)
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
