use std::collections::HashMap;
use std::io;
use rand::Rng;

fn main() {
    let periodic_table = [
        (1, "hydrogen"), (2, "helium"), (3, "lithium"), (4, "beryllium"), (5, "boron"),
        (6, "carbon"), (7, "nitrogen"), (8, "oxygen"), (9, "fluorine"), (10, "neon"),
        (11, "sodium"), (12, "magnesium"), (13, "aluminium"), (14, "silicon"), (15, "phosphorus"),
        (16, "sulphur"), (17, "chlorine"), (18, "argon"), (19, "potassium"), (20, "calcium"),
        (21, "scandium"), (22, "titanium"), (23, "vanadium"), (24, "chromium"), (25, "manganese"),
        (26, "iron"), (27, "cobalt"), (28, "nickel"), (29, "copper"), (30, "zinc"),
        (31, "gallium"), (32, "germanium"), (33, "arsenic"), (34, "selenium"), (35, "bromine"),
        (36, "krypton"), (37, "rubidium"), (38, "strontium"), (39, "yttrium"), (40, "zirconium"),
        (41, "niobium"), (42, "molybdenum"), (43, "technetium"), (44, "ruthenium"), (45, "rhodium"),
        (46, "palladium"), (47, "silver"), (48, "cadmium"), (49, "indium"), (50, "tin"),
        (51, "antimony"), (52, "tellurium"), (53, "iodine"), (54, "xenon"), (55, "caesium"),
        (56, "barium"), (57, "lanthanum"), (58, "cerium"), (59, "praseodymium"), (60, "neodymium"),
        (61, "promethium"), (62, "samarium"), (63, "europium"), (64, "gadolinium"), (65, "terbium"),
        (66, "dysprosium"), (67, "holmium"), (68, "erbium"), (69, "thulium"), (70, "ytterbium"),
        (71, "lutetium"), (72, "hafnium"), (73, "tantalum"), (74, "tungsten"), (75, "rhenium"),
        (76, "osmium"), (77, "iridium"), (78, "platinum"), (79, "gold"), (80, "mercury"),
        (81, "thallium"), (82, "lead"), (83, "bismuth"), (84, "polonium"), (85, "astatine"),
        (86, "radon"), (87, "francium"), (88, "radium"), (89, "actinium"), (90, "thorium"),
        (91, "protactinium"), (92, "uranium"), (93, "neptunium"), (94, "plutonium"), (95, "americium"),
        (96, "curium"), (97, "berkelium"), (98, "californium"), (99, "einsteinium"), (100, "fermium"),
        (101, "mendelevium"), (102, "nobelium"), (103, "lawrencium"), (104, "rutherfordium"), (105, "dubnium"),
        (106, "seaborgium"), (107, "bohrium"), (108, "hassium"), (109, "meitnerium"), (110, "darmstadtium"),
        (111, "roentgenium"), (112, "copernicium"), (113, "nihonium"), (114, "flerovium"), (115, "moscovium"),
        (116, "livermorium"), (117, "tennessine"), (118, "oganesson"),
    ].iter().cloned().collect::<HashMap<_, _>>();

    loop {
        println!("Do you want to guess by 'num' or 'elem'? Type 'exit' to quit.");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "num" => {
                let num = rand::thread_rng().gen_range(1..=118);
                println!("Guess the element for atomic number {}: ", num);
                let mut guess = String::new();
                io::stdin().read_line(&mut guess).expect("Failed to read line");
                let guess = guess.trim().to_lowercase();

                if periodic_table.get(&num).unwrap() == &guess {
                    println!("Correct!");
                } else {
                    println!("Wrong, the element for atomic number {} is {}.", num, periodic_table.get(&num).unwrap());
                }
            },
            "elem" => {
                let elem = periodic_table.values().nth(rand::thread_rng().gen_range(0..118)).unwrap();
                println!("Guess the atomic number for element {}: ", elem);
                let mut guess = String::new();
                io::stdin().read_line(&mut guess).expect("Failed to read line");
                let guess: usize = guess.trim().parse().expect("Please type a number.");

                if periodic_table.iter().find(|&(_, &v)| v == elem).unwrap().0 == &guess {
                    println!("Correct!");
                } else {
                    println!("Wrong, the atomic number for {} is {}.", elem, periodic_table.iter().find(|&(_, &v)| v == elem).unwrap().0);
                }
            },
            "exit" => break,
            _ => println!("Invalid choice, please type 'num', 'elem', or 'exit'."),
        }

        println!("Do you want to play again? (y/n)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");
        if play_again.trim().to_lowercase() != "y" {
            break;
        }
    }
}
