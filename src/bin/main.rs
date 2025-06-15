use std::io::Write;

use anyhow::{anyhow, Result};
use nums::game::{DiceAmount, Game};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if let Ok(mut game) = args_to_game(args) {
        game.print_game();
        game.solve();
        game.solutions.sort();

        if get_yn_input("Anzahl der gefundenen Lösungen anzeigen (j/n)? ").is_err() {
            return;
        }

        game.print_solution_amount();

        if game.solutions.len() == 0 {
            return;
        }

        if get_yn_input("Lösungen anzeigen (j/n)? ").is_err() {
            return;
        }

        game.print_solutions();
    }
}

fn args_to_game(args: Vec<String>) -> Result<Game> {
    return match args.len() {
        1 => {
            return Err(anyhow!("Bitte gib eine Zahl an, die erreicht werden soll."));
        }
        2 => {
            let amount_cubes = match args[1].parse::<u8>().unwrap() {
                3 => DiceAmount::Three,
                4 => DiceAmount::Four,
                _ => {
                    return Err(anyhow!("Nur 3 oder 4 Würfel sind erlaubt."));
                }
            };
            Ok(Game::new(amount_cubes))
        }
        3 => {
            let amount_cubes = match args[1].parse::<u8>().unwrap() {
                3 => DiceAmount::Three,
                4 => DiceAmount::Four,
                _ => {
                    return Err(anyhow!("Nur 3 oder 4 Würfel sind erlaubt."));
                }
            };
            let num = args[2]
                .parse::<u64>()
                .expect("Argument <Nummer> muss eine Zahl sein");

            Ok(Game::of_number(amount_cubes, num))
        }

        6 | 7 => {
            let amount_cubes = match args[1].parse::<u8>().unwrap() {
                3 => DiceAmount::Three,
                4 => DiceAmount::Four,
                _ => {
                    return Err(anyhow!("Nur 3 oder 4 Würfel sind erlaubt."));
                }
            };
            let num = args[2]
                .parse::<u64>()
                .expect("Argument <Nummer> muss eine Zahl sein");

            let mut dices = [0; 4];
            dices[0] = args[3]
                .parse::<u64>()
                .expect("Argument <Würfel1> muss eine Zahl sein");

            dices[1] = args[4]
                .parse::<u64>()
                .expect("Argument <Würfel2> muss eine Zahl sein");

            dices[2] = args[5]
                .parse::<u64>()
                .expect("Argument <Würfel2> muss eine Zahl sein");

            if args.len() > 6 {
                dices[3] = args[6]
                    .parse::<u64>()
                    .expect("Argument <Würfel2> muss eine Zahl sein");
            }

            Ok(Game::of(amount_cubes, num, dices))
        }
        _ => Err(anyhow!(help())),
    };
}

fn help() -> String {
    let mut help = String::new();
    help.push_str("Aufruf: ");
    help.push_str("./nums <Nummer> <Würfel1> <Würfel2> <Würfel3> <Würfel4>\n");
    help.push_str("Möglichkeiten:\n");
    help.push_str(" - 0 Argumente: Zufällige Zahl und Würfel\n");
    help.push_str(" - 1 Argument: Vorgegebene Zahl und zufällige Würfel\n");
    help.push_str(" - 5 Argumente: Vorgegebene Zahl und Würfel\n");
    help
}

fn get_yn_input(question: &str) -> Result<()> {
    'outer: loop {
        print!("{}", question);
        if let Err(e) = std::io::stdout().flush() {
            return Err(anyhow!(e));
        }

        loop {
            let mut character = String::new();
            if let Ok(_) = std::io::stdin().read_line(&mut character) {
                if character.len() != 2 {
                    continue 'outer;
                }

                if let Some(c) = character.chars().next() {
                    match c {
                        'j' => return Ok(()),
                        'n' => return Err(anyhow!("Abbruch durch Nutzer")),
                        _ => continue 'outer,
                    }
                }
            }
        }
    }
}
