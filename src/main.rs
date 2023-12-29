use rand::Rng;
use std::{
    collections::HashMap,
    io::{self, stdin, Read, Write},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut game = match args.len() {
        1 => Game::new(),
        2 => Game::of_number(
            args[1]
                .parse::<u64>()
                .expect("Argument <Nummer> muss eine Zahl sein"),
        ),
        6 => Game::of(
            args[1]
                .parse::<u64>()
                .expect("Argument <Nummer> muss eine Zahl sein"),
            [
                args[2]
                    .parse::<u64>()
                    .expect("Argument <Würfel1> muss eine Zahl sein"),
                args[3]
                    .parse::<u64>()
                    .expect("Argument <Würfel2> muss eine Zahl sein"),
                args[4]
                    .parse::<u64>()
                    .expect("Argument <Würfel3> muss eine Zahl sein"),
                args[5]
                    .parse::<u64>()
                    .expect("Argument <Würfel4> muss eine Zahl sein"),
            ],
        ),
        _ => {
            println!("Aufruf: ");
            println!("./nums <Nummer> <Würfel1> <Würfel2> <Würfel3> <Würfel4>");
            println!("Möglichkeiten:");
            println!(" - 0 Argumente: Zufällige Zahl und Würfel");
            println!(" - 1 Argument: Vorgegebene Zahl und zufällige Würfel");
            println!(" - 4 Argumente: Vorgegebene Zahl und Würfel");
            return;
        }
    };

    println!("{}", game);
    game.solve();
    game.solutions.sort_by(compare_solutions_by_score);

    'outer: loop {
        print!("Anzahl der gefundenen Lösungen anzeigen (j/n)? ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Konsolenfehler {}", e);
            return;
        }

        loop {
            let mut character = String::new();
            if let Ok(_) = stdin().read_line(&mut character) {
                if character.len() != 2 {
                    continue 'outer;
                }

                if let Some(c) = character.chars().next() {
                    match c {
                        'j' => {
                            game.print_solution_amount();
                            break 'outer;
                        }
                        'n' => return,
                        _ => continue 'outer,
                    }
                }
            }
        }
    }

    'outer: loop {
        print!("Lösungen anzeigen (j/n)? ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Konsolenfehler {}", e);
            return;
        }
        loop {
            let mut character = String::new();
            if let Ok(_) = stdin().read_line(&mut character) {
                if character.len() != 2 {
                    continue 'outer;
                }

                if let Some(c) = character.chars().next() {
                    match c {
                        'j' => {
                            game.print_solutions();
                            break 'outer;
                        }
                        'n' => return,
                        _ => continue 'outer,
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for Calculation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Calculation::Add(a, b) => write!(f, "({} + {})", a, b),
            Calculation::Sub(a, b) => write!(f, "({} - {})", a, b),
            Calculation::Mul(a, b) => write!(f, "({} * {})", a, b),
            Calculation::Div(a, b) => write!(f, "({} / {})", a, b),
            Calculation::Cube(_, c) => write!(f, "{}", c),
        }
    }
}

struct Game {
    num: u64,
    dices: [u64; 4],
    solutions: Vec<Calculation>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum UsedCubes {
    OneCube(u64),
    TwoCubes(u64, u64),
    ThreeCubes(u64, u64, u64),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Calculation {
    Add(Box<Calculation>, Box<Calculation>),
    Sub(Box<Calculation>, Box<Calculation>),
    Mul(Box<Calculation>, Box<Calculation>),
    Div(Box<Calculation>, Box<Calculation>),
    Cube(usize, u64),
}

impl Game {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let num: u64 = rng.gen_range(100..=999);

        let dices = [
            rng.gen_range(1..=6),
            rng.gen_range(1..=6),
            rng.gen_range(1..=6),
            rng.gen_range(1..=6),
        ];

        Self {
            num,
            dices,
            solutions: Vec::new(),
        }
    }

    fn of(n: u64, dices: [u64; 4]) -> Self {
        Self {
            num: n,
            dices,
            solutions: Vec::new(),
        }
    }

    fn of_number(num: u64) -> Self {
        let mut rng = rand::thread_rng();
        let dices = [
            rng.gen_range(1..=6),
            rng.gen_range(1..=6),
            rng.gen_range(1..=6),
            rng.gen_range(1..=6),
        ];

        Self {
            num,
            dices,
            solutions: Vec::new(),
        }
    }

    fn solve(&mut self) {
        let (tx, rx): (Sender<Calculation>, Receiver<Calculation>) = mpsc::channel();
        let n = self.num.clone();
        let ds = self.dices.clone();
        thread::spawn(move || {
            let mut dp: HashMap<UsedCubes, HashMap<u64, Calculation>> = HashMap::new();

            // single cubes
            for (i, c) in ds.iter().enumerate() {
                let mut map: HashMap<u64, Calculation> = HashMap::new();
                map.insert(*c, Calculation::Cube(i, *c));
                map.insert((*c) * 10, Calculation::Cube(i, (*c) * 10));
                map.insert((*c) * 100, Calculation::Cube(i, (*c) * 100));
                map.insert((*c) * 1000, Calculation::Cube(i, (*c) * 1000));

                map.insert((*c) * 10000, Calculation::Cube(i, (*c) * 10000));
                map.insert((*c) * 100000, Calculation::Cube(i, (*c) * 100000));
                map.insert((*c) * 1000000, Calculation::Cube(i, (*c) * 1000000));
                map.insert((*c) * 10000000, Calculation::Cube(i, (*c) * 10000000));

                dp.insert(UsedCubes::OneCube(i as u64), map);
            }

            // two cubes
            let mut maps = Vec::new();

            for (c1, map1) in dp.iter() {
                for (c2, map2) in dp.iter() {
                    let UsedCubes::OneCube(c1) = c1 else { panic!() };
                    let UsedCubes::OneCube(c2) = c2 else { panic!() };

                    if c1 <= c2 {
                        continue;
                    }

                    let map: HashMap<u64, Calculation> = calculate_result_map(map1, map2);
                    maps.push((UsedCubes::TwoCubes(*c1, *c2), map));
                }
            }

            while let Some((c, m)) = maps.pop() {
                dp.insert(c, m);
            }

            // result of calculation of two cubes twice
            for (cubes1, map1) in dp.iter() {
                for (cubes2, map2) in dp.iter() {
                    if let UsedCubes::TwoCubes(c1, c2) = cubes1 {
                        if let UsedCubes::TwoCubes(c3, c4) = cubes2 {
                            if c1 == c3 || c1 == c4 || c2 == c3 || c2 == c4 {
                                continue;
                            }

                            check_for_solutions(map1, map2, tx.clone(), n.clone());
                        }
                    }
                }
            }

            // three cubes
            let mut maps = Vec::new();
            for (cubes1, map1) in dp.iter() {
                if let UsedCubes::OneCube(c1) = cubes1 {
                    for (cubes2, map2) in dp.iter() {
                        if let UsedCubes::TwoCubes(c2, c3) = cubes2 {
                            if c1 == c2 || c1 == c3 {
                                continue;
                            }

                            let map = calculate_result_map(map1, map2);
                            maps.push((UsedCubes::ThreeCubes(*c1, *c2, *c3), map));
                        }
                    }
                }
            }

            while let Some((c, m)) = maps.pop() {
                dp.insert(c, m);
            }

            // result of calculation of three cubes and single cube
            for (cubes1, map1) in dp.iter() {
                if let UsedCubes::OneCube(c1) = cubes1 {
                    for (cubes2, map2) in dp.iter() {
                        if let UsedCubes::ThreeCubes(c2, c3, c4) = cubes2 {
                            if c1 == c2 || c1 == c3 || c1 == c4 {
                                continue;
                            }

                            check_for_solutions(map1, map2, tx.clone(), n.clone());
                        }
                    }
                }
            }
        });

        for received in rx {
            if self.solutions.contains(&received) {
                continue;
            }
            self.solutions.push(received);
        }
    }

    fn print_solution_amount(&self) {
        println!("\n{} Lösungen gefunden\n", self.solutions.len());
    }

    fn print_solutions(&self) {
        println!("\nEinfachste Lösung: {}", self.solutions[0]);
        println!(
            "Schwierigste Lösung: {}\n",
            self.solutions[self.solutions.len() - 1]
        );

        println!("Alle {} Lösungen:", self.solutions.len());
        for solution in &self.solutions {
            println!("\t{} = {}", solution, self.num);
        }
        println!("\n");
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Zahl: {}\nWürfel: {} {} {} {}",
            self.num, self.dices[0], self.dices[1], self.dices[2], self.dices[3]
        )
    }
}

fn calculate_result_map(
    map1: &HashMap<u64, Calculation>,
    map2: &HashMap<u64, Calculation>,
) -> HashMap<u64, Calculation> {
    let mut result_map = HashMap::new();

    for (res1, calc1) in map1.iter() {
        for (res2, calc2) in map2.iter() {
            if let Some(add) = res1.checked_add(*res2) {
                result_map.insert(
                    add,
                    Calculation::Add(Box::new(calc1.clone()), Box::new(calc2.clone())),
                );
            }

            if res1 >= res2 {
                if let Some(sub) = res1.checked_sub(*res2) {
                    result_map.insert(
                        sub,
                        Calculation::Sub(Box::new(calc1.clone()), Box::new(calc2.clone())),
                    );
                }
            }
            if res2 >= res1 {
                if let Some(sub) = res2.checked_sub(*res1) {
                    result_map.insert(
                        sub,
                        Calculation::Sub(Box::new(calc2.clone()), Box::new(calc1.clone())),
                    );
                }
            }

            if let Some(mult) = res1.checked_mul(*res2) {
                result_map.insert(
                    mult,
                    Calculation::Mul(Box::new(calc1.clone()), Box::new(calc2.clone())),
                );
            }

            if *res2 > 0 {
                if *res1 % 10 == 0 && *res2 % 10 == 0 && *res1 > 0 {
                    continue;
                }
                if let Some(div) = res1.checked_div(*res2) {
                    if div * *res2 == *res1 {
                        result_map.insert(
                            div,
                            Calculation::Div(Box::new(calc1.clone()), Box::new(calc2.clone())),
                        );
                    }
                }
            }
            if *res1 > 0 {
                if *res1 % 10 == 0 && *res2 % 10 == 0 && *res2 > 0 {
                    continue;
                }
                if let Some(div) = res2.checked_div(*res1) {
                    if div * *res1 == *res2 {
                        result_map.insert(
                            div,
                            Calculation::Div(Box::new(calc2.clone()), Box::new(calc1.clone())),
                        );
                    }
                }
            }
        }
    }

    return result_map;
}

fn check_for_solutions(
    map1: &HashMap<u64, Calculation>,
    map2: &HashMap<u64, Calculation>,
    tx: Sender<Calculation>,
    n: u64,
) {
    for (r1, calc1) in map1 {
        for (r2, calc2) in map2 {
            if let Some(add) = r1.checked_add(*r2) {
                if add == n {
                    tx.send(Calculation::Add(
                        Box::new(calc1.clone()),
                        Box::new(calc2.clone()),
                    ))
                    .expect("Lösung konnte nicht gesendet werden");
                }
            }

            if r1 >= r2 {
                if let Some(sub) = r1.checked_sub(*r2) {
                    if sub == n {
                        tx.send(Calculation::Sub(
                            Box::new(calc1.clone()),
                            Box::new(calc2.clone()),
                        ))
                        .expect("Lösung konnte nicht gesendet werden");
                    }
                }
            }

            if r2 >= r1 {
                if let Some(sub) = r2.checked_sub(*r1) {
                    if sub == n {
                        tx.send(Calculation::Sub(
                            Box::new(calc2.clone()),
                            Box::new(calc1.clone()),
                        ))
                        .expect("Lösung konnte nicht gesendet werden");
                    }
                }
            }

            if let Some(mult) = r1.checked_mul(*r2) {
                if mult == n {
                    tx.send(Calculation::Mul(
                        Box::new(calc1.clone()),
                        Box::new(calc2.clone()),
                    ))
                    .expect("Lösung konnte nicht gesendet werden");
                }
            }

            if *r2 > 0 {
                if *r1 % 10 == 0 && *r2 % 10 == 0 && *r1 > 0 {
                    continue;
                }
                if let Some(div) = r1.checked_div(*r2) {
                    if div * *r2 == *r1 {
                        if div as u64 == n {
                            tx.send(Calculation::Div(
                                Box::new(calc1.clone()),
                                Box::new(calc2.clone()),
                            ))
                            .expect("Lösung konnte nicht gesendet werden");
                        }
                    }
                }
            }
            if *r1 > 0 {
                if *r1 % 10 == 0 && *r2 % 10 == 0 && *r2 > 0 {
                    continue;
                }
                if let Some(div) = r2.checked_div(*r1) {
                    if div * *r1 == *r2 {
                        if div as u64 == n {
                            tx.send(Calculation::Div(
                                Box::new(calc2.clone()),
                                Box::new(calc1.clone()),
                            ))
                            .expect("Lösung konnte nicht gesendet werden");
                        }
                    }
                }
            }
        }
    }
}

fn score_calculation(calc: &Calculation) -> u32 {
    match calc {
        Calculation::Add(a, b) => 2 + score_calculation(a) + score_calculation(b),
        Calculation::Sub(a, b) => 2 + score_calculation(a) + score_calculation(b),
        Calculation::Mul(a, b) => 3 + score_calculation(a) + score_calculation(b),
        Calculation::Div(a, b) => 3 + score_calculation(a) + score_calculation(b),
        Calculation::Cube(_, _) => 1,
    }
}

fn compare_solutions_by_score(a: &Calculation, b: &Calculation) -> std::cmp::Ordering {
    let score_a = score_calculation(a);
    let score_b = score_calculation(b);

    if score_a <= score_b {
        return std::cmp::Ordering::Less;
    } else {
        return std::cmp::Ordering::Greater;
    }
}
