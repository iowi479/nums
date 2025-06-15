use anyhow::Result;
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
};

use crate::{game::calculation::Calculation, output::Output};

mod calculation;

pub struct Game {
    pub num_dices: DiceAmount,
    pub num: u64,
    pub dices: [u64; 4],
    pub solutions: Vec<Calculation>,
}

pub enum DiceAmount {
    Four,
    Three,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum UsedCubes {
    OneCube(u64),
    TwoCubes(u64, u64),
    ThreeCubes(u64, u64, u64),
}

impl Game {
    pub fn new(num_dices: DiceAmount) -> Self {
        let num: u64 = match num_dices {
            DiceAmount::Three => rand::random_range(1..=99),
            DiceAmount::Four => rand::random_range(100..=999),
        };

        let dices = [
            rand::random_range(1..=6),
            rand::random_range(1..=6),
            rand::random_range(1..=6),
            rand::random_range(1..=6),
        ];

        Self {
            num_dices,
            num,
            dices,
            solutions: Vec::new(),
        }
    }

    pub fn of(num_dices: DiceAmount, num: u64, dices: [u64; 4]) -> Self {
        Self {
            num_dices,
            num,
            dices,
            solutions: Vec::new(),
        }
    }

    pub fn of_number(num_dices: DiceAmount, num: u64) -> Self {
        let dices = [
            rand::random_range(1..=6),
            rand::random_range(1..=6),
            rand::random_range(1..=6),
            rand::random_range(1..=6),
        ];

        Self {
            num_dices,
            num,
            dices,
            solutions: Vec::new(),
        }
    }

    pub fn solve(&mut self) {
        match self.num_dices {
            DiceAmount::Three => self.solve_three(),
            DiceAmount::Four => self.solve_four(),
        }
    }

    fn solve_three(&mut self) {
        let (tx, rx): (Sender<Calculation>, Receiver<Calculation>) = std::sync::mpsc::channel();
        let n = self.num.clone();
        let mut ds: [u64; 3] = [0; 3];
        ds.copy_from_slice(&self.dices[0..3]);

        std::thread::spawn(move || {
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

            // three cubes
            for (cubes1, map1) in dp.iter() {
                if let UsedCubes::OneCube(c1) = cubes1 {
                    for (cubes2, map2) in dp.iter() {
                        if let UsedCubes::TwoCubes(c2, c3) = cubes2 {
                            if c1 == c2 || c1 == c3 {
                                continue;
                            }

                            if let Err(e) = check_for_solutions(map1, map2, tx.clone(), n.clone()) {
                                eprintln!("Kanalfehler {}", e);
                            }
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

    fn solve_four(&mut self) {
        let (tx, rx): (Sender<Calculation>, Receiver<Calculation>) = std::sync::mpsc::channel();
        let n = self.num.clone();
        let ds = self.dices.clone();
        std::thread::spawn(move || {
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

                            if let Err(e) = check_for_solutions(map1, map2, tx.clone(), n.clone()) {
                                eprintln!("Kanalfehler {}", e);
                            }
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

                            if let Err(e) = check_for_solutions(map1, map2, tx.clone(), n.clone()) {
                                eprintln!("Kanalfehler {}", e);
                            }
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

    pub fn print_game(&self) {
        println!("{}", self);
    }

    pub fn print_solution_amount(&self) {
        println!("\n{} Lösungen gefunden\n", self.solutions.len());
    }

    pub fn print_solutions(&self) {
        if self.solutions.len() >= 2 {
            println!("\nEinfachste Lösung: {}", self.solutions[0]);
            println!(
                "Schwierigste Lösung: {}\n",
                self.solutions[self.solutions.len() - 1]
            );
        } else {
            println!("");
        }

        println!("Alle {} Lösungen:", self.solutions.len());
        for solution in &self.solutions {
            println!("\t{} = {}", solution, self.num);
        }
        println!("\n");
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
                if res1 >= res2 {
                    result_map.insert(
                        add,
                        Calculation::Add(Box::new(calc1.clone()), Box::new(calc2.clone())),
                    );
                } else {
                    result_map.insert(
                        add,
                        Calculation::Add(Box::new(calc2.clone()), Box::new(calc1.clone())),
                    );
                }
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
                if res1 >= res2 {
                    result_map.insert(
                        mult,
                        Calculation::Mul(Box::new(calc1.clone()), Box::new(calc2.clone())),
                    );
                } else {
                    result_map.insert(
                        mult,
                        Calculation::Mul(Box::new(calc2.clone()), Box::new(calc1.clone())),
                    );
                }
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
) -> Result<()> {
    for (r1, calc1) in map1 {
        for (r2, calc2) in map2 {
            if let Some(add) = r1.checked_add(*r2) {
                if add == n {
                    tx.send(Calculation::Add(
                        Box::new(calc1.clone()),
                        Box::new(calc2.clone()),
                    ))?;
                }
            }

            if r1 >= r2 {
                if let Some(sub) = r1.checked_sub(*r2) {
                    if sub == n {
                        tx.send(Calculation::Sub(
                            Box::new(calc1.clone()),
                            Box::new(calc2.clone()),
                        ))?;
                    }
                }
            }

            if r2 >= r1 {
                if let Some(sub) = r2.checked_sub(*r1) {
                    if sub == n {
                        tx.send(Calculation::Sub(
                            Box::new(calc2.clone()),
                            Box::new(calc1.clone()),
                        ))?;
                    }
                }
            }

            if let Some(mult) = r1.checked_mul(*r2) {
                if mult == n {
                    tx.send(Calculation::Mul(
                        Box::new(calc1.clone()),
                        Box::new(calc2.clone()),
                    ))?;
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
                            ))?;
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
                            ))?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dices = match self.num_dices {
            DiceAmount::Three => &self.dices[0..3],
            DiceAmount::Four => &self.dices,
        };

        write!(
            f,
            "\n{}\n\n{}\n",
            Output::number_string(self.num),
            Output::dices_string(dices)
        )
    }
}
