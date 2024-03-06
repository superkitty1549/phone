struct TowersOfHanoi {
    pins: Vec<Vec<usize>>,
    num_disks: usize,
}

impl TowersOfHanoi {
    fn new(num_disks: usize) -> TowersOfHanoi {
        let mut pins = vec![Vec::new(); 3];
        for i in (0..num_disks).rev() {
            pins[0].push(i + 1);
        }
        TowersOfHanoi { pins, num_disks }
    }

    fn display(&self) {
        println!();
        for i in (0..self.num_disks).rev() {
            for pin in &self.pins {
                match pin.get(i) {
                    Some(&disk) => print!("{} ", disk),
                    None => print!("0 "),
                }
            }
            println!();
        }
        println!();
    }

    fn move_disk(&mut self, from: usize, to: usize) -> bool {
        if from >= 3 || to >= 3 {
            return false;
        }

        match (self.pins[from].last(), self.pins[to].last()) {
            (Some(&disk), None) => {
                self.pins[to].push(disk);
                self.pins[from].pop();
                true
            }
            (Some(&disk1), Some(&disk2)) if disk1 < disk2 => {
                self.pins[to].push(disk1);
                self.pins[from].pop();
                true
            }
            _ => {
                false
            }
        }
    }

    fn is_solved(&self) -> bool {
        self.pins.get(2).unwrap().len() == self.num_disks
    }
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

fn main() {
    println!("tohn V.0.0.1\n`[from] [to]` to move disc\n`q` to quit");

    let num_disks: usize = std::env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::new())
        .parse::<usize>()
        .unwrap_or(3);

    let mut game: TowersOfHanoi = TowersOfHanoi::new(num_disks);

    loop {
        game.display();

        let input: String = get_input(">> ");
        if input == String::from("q") {
            println!("okie bye bye");
            break;
        } 

        let mut parts = input.split_whitespace();
        let from = parts.next().unwrap_or_default().parse::<usize>().unwrap_or(4);
        let to = parts.next().unwrap_or_default().parse::<usize>().unwrap_or(4);

        if !game.move_disk(from, to) {
            println!("Nuh uh");
        }

        if game.is_solved() {
            println!("Yay u win :D");
            break;
        }
    }
}

