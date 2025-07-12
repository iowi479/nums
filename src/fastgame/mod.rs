use crate::{fastgame::calculation::Calculation, output::Output};
use std::collections::{HashMap, HashSet};

mod calculation;

const MAX_DICES: usize = 4;
type DiceValue = u32;

#[derive(Debug, Clone)]
pub struct Game {
    pub num_dices: DiceAmount,
    pub num: DiceValue,
    pub dices: [u8; MAX_DICES],
    pub solutions: HashSet<Calculation>,

    pub result_map_pool: Vec<HashMap<DiceValue, Calculation>>,
    pub dp: HashMap<UsedCubes, HashMap<DiceValue, Calculation>>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum DiceAmount {
    Four,
    Three,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum UsedCubes {
    OneCube(DiceValue),
    TwoCubes(DiceValue, DiceValue),
    ThreeCubes(DiceValue, DiceValue, DiceValue),
}

impl Game {
    pub fn new(num_dices: DiceAmount) -> Self {
        Self {
            num_dices,
            num: 0,
            dices: [0u8; MAX_DICES],
            solutions: HashSet::new(),

            result_map_pool: Vec::new(),
            dp: HashMap::new(),
        }
    }

    pub fn solve(&mut self, value: DiceValue, dices: [u8; MAX_DICES]) {
        self.num = value;
        self.dices = dices;
        self.solutions.clear();
        self.dp.clear();

        match self.num_dices {
            DiceAmount::Three => self.solve_three(),
            DiceAmount::Four => self.solve_four(),
        }
    }

    pub fn solve_three_fast(
        &mut self,
        min: u16,
        max: u16,
        dices: [u8; MAX_DICES],
        solutions: &mut Vec<u16>,
    ) {
        self.solutions.clear();
        self.dp.clear();
        let mut ds: [DiceValue; 3] = [0; 3];
        ds[0] = dices[0] as DiceValue;
        ds[1] = dices[1] as DiceValue;
        ds[2] = dices[2] as DiceValue;

        let mut checking = HashSet::new();
        let mut found = HashSet::new();
        for i in min..max {
            checking.insert(i);
        }

        // single cubes
        for (i, c) in ds.iter().enumerate() {
            let mut map: HashMap<DiceValue, Calculation> = self
                .result_map_pool
                .pop()
                .or_else(|| Some(HashMap::new()))
                .unwrap();

            map.insert(*c, Calculation::Cube(i, *c));
            map.insert((*c) * 10, Calculation::Cube(i, (*c) * 10));
            map.insert((*c) * 100, Calculation::Cube(i, (*c) * 100));
            map.insert((*c) * 1000, Calculation::Cube(i, (*c) * 1000));

            map.insert((*c) * 10000, Calculation::Cube(i, (*c) * 10000));
            map.insert((*c) * 100000, Calculation::Cube(i, (*c) * 100000));
            map.insert((*c) * 1000000, Calculation::Cube(i, (*c) * 1000000));
            map.insert((*c) * 10000000, Calculation::Cube(i, (*c) * 10000000));

            self.dp.insert(UsedCubes::OneCube(i as DiceValue), map);
        }

        // two cubes
        let mut maps = Vec::new();

        for (c1, map1) in self.dp.iter() {
            for (c2, map2) in self.dp.iter() {
                let UsedCubes::OneCube(c1) = c1 else { panic!() };
                let UsedCubes::OneCube(c2) = c2 else { panic!() };

                if c1 <= c2 {
                    continue;
                }

                let mut map: HashMap<DiceValue, Calculation> = self
                    .result_map_pool
                    .pop()
                    .or_else(|| Some(HashMap::new()))
                    .unwrap();

                calculate_result_map(map1, map2, &mut map);
                maps.push((UsedCubes::TwoCubes(*c1, *c2), map));
            }
        }

        self.dp.extend(maps);

        // three cubes
        for (cubes1, map1) in self.dp.iter() {
            if let UsedCubes::OneCube(c1) = cubes1 {
                for (cubes2, map2) in self.dp.iter() {
                    if let UsedCubes::TwoCubes(c2, c3) = cubes2 {
                        if c1 == c2 || c1 == c3 {
                            continue;
                        }

                        checking.retain(|x| {
                            if has_solution(map1, map2, *x as DiceValue) {
                                let _ = found.insert(*x);
                                false
                            } else {
                                true
                            }
                        });
                    }
                }
            }
        }

        for (_, mut map1) in self.dp.drain() {
            map1.clear();
            self.result_map_pool.push(map1);
        }

        let mut found = found.iter().collect::<Vec<_>>();
        found.sort();
        solutions.clear();
        solutions.extend(found.iter().map(|&&x| x as u16));
    }

    pub fn solve_four_fast(
        &mut self,
        min: u16,
        max: u16,
        dices: [u8; MAX_DICES],
        solutions: &mut Vec<u16>,
    ) {
        self.solutions.clear();
        self.dp.clear();
        let mut ds: [DiceValue; 4] = [0; 4];
        ds[0] = dices[0] as DiceValue;
        ds[1] = dices[1] as DiceValue;
        ds[2] = dices[2] as DiceValue;
        ds[3] = dices[3] as DiceValue;

        let mut checking = HashSet::new();
        let mut found = HashSet::new();
        for i in min..max {
            checking.insert(i);
        }

        let mut dp: HashMap<UsedCubes, HashMap<DiceValue, Calculation>> = HashMap::new();

        // single cubes
        for (i, c) in ds.iter().enumerate() {
            let mut map: HashMap<DiceValue, Calculation> = HashMap::new();
            map.insert(*c, Calculation::Cube(i, *c));
            map.insert((*c) * 10, Calculation::Cube(i, (*c) * 10));
            map.insert((*c) * 100, Calculation::Cube(i, (*c) * 100));
            map.insert((*c) * 1000, Calculation::Cube(i, (*c) * 1000));

            map.insert((*c) * 10000, Calculation::Cube(i, (*c) * 10000));
            map.insert((*c) * 100000, Calculation::Cube(i, (*c) * 100000));
            map.insert((*c) * 1000000, Calculation::Cube(i, (*c) * 1000000));
            map.insert((*c) * 10000000, Calculation::Cube(i, (*c) * 10000000));

            dp.insert(UsedCubes::OneCube(i as DiceValue), map);
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

                let mut map: HashMap<DiceValue, Calculation> = self
                    .result_map_pool
                    .pop()
                    .or_else(|| Some(HashMap::new()))
                    .unwrap();
                calculate_result_map(map1, map2, &mut map);
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

                        checking.retain(|x| {
                            if has_solution(map1, map2, *x as DiceValue) {
                                let _ = found.insert(*x);
                                false
                            } else {
                                true
                            }
                        });
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

                        let mut map: HashMap<DiceValue, Calculation> = self
                            .result_map_pool
                            .pop()
                            .or_else(|| Some(HashMap::new()))
                            .unwrap();
                        calculate_result_map(map1, map2, &mut map);
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

                        checking.retain(|x| {
                            if has_solution(map1, map2, *x as DiceValue) {
                                let _ = found.insert(*x);
                                false
                            } else {
                                true
                            }
                        });
                    }
                }
            }
        }

        for (_, mut map1) in self.dp.drain() {
            map1.clear();
            self.result_map_pool.push(map1);
        }

        let mut found = found.iter().collect::<Vec<_>>();
        found.sort();
        solutions.clear();
        solutions.extend(found.iter().map(|&&x| x as u16));
    }

    fn solve_three(&mut self) {
        let n = self.num.clone();
        let mut ds: [DiceValue; 3] = [0; 3];
        ds[0] = self.dices[0] as DiceValue;
        ds[1] = self.dices[1] as DiceValue;
        ds[2] = self.dices[2] as DiceValue;

        // single cubes
        for (i, c) in ds.iter().enumerate() {
            let mut map: HashMap<DiceValue, Calculation> = self
                .result_map_pool
                .pop()
                .or_else(|| Some(HashMap::new()))
                .unwrap();

            map.insert(*c, Calculation::Cube(i, *c));
            map.insert((*c) * 10, Calculation::Cube(i, (*c) * 10));
            map.insert((*c) * 100, Calculation::Cube(i, (*c) * 100));
            map.insert((*c) * 1000, Calculation::Cube(i, (*c) * 1000));

            map.insert((*c) * 10000, Calculation::Cube(i, (*c) * 10000));
            map.insert((*c) * 100000, Calculation::Cube(i, (*c) * 100000));
            map.insert((*c) * 1000000, Calculation::Cube(i, (*c) * 1000000));
            map.insert((*c) * 10000000, Calculation::Cube(i, (*c) * 10000000));

            self.dp.insert(UsedCubes::OneCube(i as DiceValue), map);
        }

        // two cubes
        let mut maps = Vec::new();

        for (c1, map1) in self.dp.iter() {
            for (c2, map2) in self.dp.iter() {
                let UsedCubes::OneCube(c1) = c1 else { panic!() };
                let UsedCubes::OneCube(c2) = c2 else { panic!() };

                if c1 <= c2 {
                    continue;
                }

                let mut map: HashMap<DiceValue, Calculation> = self
                    .result_map_pool
                    .pop()
                    .or_else(|| Some(HashMap::new()))
                    .unwrap();

                calculate_result_map(map1, map2, &mut map);
                maps.push((UsedCubes::TwoCubes(*c1, *c2), map));
            }
        }

        self.dp.extend(maps);

        // three cubes
        for (cubes1, map1) in self.dp.iter() {
            if let UsedCubes::OneCube(c1) = cubes1 {
                for (cubes2, map2) in self.dp.iter() {
                    if let UsedCubes::TwoCubes(c2, c3) = cubes2 {
                        if c1 == c2 || c1 == c3 {
                            continue;
                        }

                        check_for_solutions(map1, map2, &mut self.solutions, n.clone());
                    }
                }
            }
        }

        for (_, mut map1) in self.dp.drain() {
            map1.clear();
            self.result_map_pool.push(map1);
        }
    }

    fn solve_four(&mut self) {
        let n = self.num.clone();
        let mut ds: [DiceValue; 4] = [0; 4];
        ds[0] = self.dices[0] as DiceValue;
        ds[1] = self.dices[1] as DiceValue;
        ds[2] = self.dices[2] as DiceValue;
        ds[3] = self.dices[3] as DiceValue;

        let mut dp: HashMap<UsedCubes, HashMap<DiceValue, Calculation>> = HashMap::new();

        // single cubes
        for (i, c) in ds.iter().enumerate() {
            let mut map: HashMap<DiceValue, Calculation> = HashMap::new();
            map.insert(*c, Calculation::Cube(i, *c));
            map.insert((*c) * 10, Calculation::Cube(i, (*c) * 10));
            map.insert((*c) * 100, Calculation::Cube(i, (*c) * 100));
            map.insert((*c) * 1000, Calculation::Cube(i, (*c) * 1000));

            map.insert((*c) * 10000, Calculation::Cube(i, (*c) * 10000));
            map.insert((*c) * 100000, Calculation::Cube(i, (*c) * 100000));
            map.insert((*c) * 1000000, Calculation::Cube(i, (*c) * 1000000));
            map.insert((*c) * 10000000, Calculation::Cube(i, (*c) * 10000000));

            dp.insert(UsedCubes::OneCube(i as DiceValue), map);
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

                let mut map: HashMap<DiceValue, Calculation> = self
                    .result_map_pool
                    .pop()
                    .or_else(|| Some(HashMap::new()))
                    .unwrap();
                calculate_result_map(map1, map2, &mut map);
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

                        check_for_solutions(map1, map2, &mut self.solutions, n.clone());
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

                        let mut map: HashMap<DiceValue, Calculation> = self
                            .result_map_pool
                            .pop()
                            .or_else(|| Some(HashMap::new()))
                            .unwrap();
                        calculate_result_map(map1, map2, &mut map);
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

                        check_for_solutions(map1, map2, &mut self.solutions, n.clone());
                    }
                }
            }
        }
    }

    pub fn print_game(&self) {
        println!("{}", self);
    }

    pub fn print_solution_amount(&self) {
        println!("\n{} Lösungen gefunden\n", self.solutions.len());
    }

    pub fn print_solutions(&self) {
        let mut s = self.solutions.iter().collect::<Vec<_>>();
        s.sort_by(|a, b| a.score().cmp(&b.score()));

        if s.len() >= 2 {
            println!("\nEinfachste Lösung: {}", s[0]);
            println!("Schwierigste Lösung: {}\n", s[s.len() - 1]);
        } else {
            println!("");
        }

        println!("Alle {} Lösungen:", s.len());
        for solution in &s {
            println!("\t{} = {}", solution, self.num);
        }
        println!("\n");
    }
}

fn calculate_result_map(
    map1: &HashMap<DiceValue, Calculation>,
    map2: &HashMap<DiceValue, Calculation>,
    result_map: &mut HashMap<DiceValue, Calculation>,
) {
    // TODO:
    assert!(result_map.is_empty());
    result_map.clear();

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
}

fn check_for_solutions(
    map1: &HashMap<DiceValue, Calculation>,
    map2: &HashMap<DiceValue, Calculation>,
    solutions: &mut HashSet<Calculation>,
    n: DiceValue,
) {
    for (r1, calc1) in map1 {
        for (r2, calc2) in map2 {
            if let Some(add) = r1.checked_add(*r2) {
                if add == n {
                    solutions.insert(Calculation::Add(
                        Box::new(calc1.clone()),
                        Box::new(calc2.clone()),
                    ));
                }
            }

            if r1 >= r2 {
                if let Some(sub) = r1.checked_sub(*r2) {
                    if sub == n {
                        solutions.insert(Calculation::Sub(
                            Box::new(calc1.clone()),
                            Box::new(calc2.clone()),
                        ));
                    }
                }
            }

            if r2 >= r1 {
                if let Some(sub) = r2.checked_sub(*r1) {
                    if sub == n {
                        solutions.insert(Calculation::Sub(
                            Box::new(calc2.clone()),
                            Box::new(calc1.clone()),
                        ));
                    }
                }
            }

            if let Some(mult) = r1.checked_mul(*r2) {
                if mult == n {
                    solutions.insert(Calculation::Mul(
                        Box::new(calc1.clone()),
                        Box::new(calc2.clone()),
                    ));
                }
            }

            if *r2 > 0 {
                if *r1 % 10 == 0 && *r2 % 10 == 0 && *r1 > 0 {
                    continue;
                }
                if let Some(div) = r1.checked_div(*r2) {
                    if div * *r2 == *r1 {
                        if div as DiceValue == n {
                            solutions.insert(Calculation::Div(
                                Box::new(calc1.clone()),
                                Box::new(calc2.clone()),
                            ));
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
                        if div as DiceValue == n {
                            solutions.insert(Calculation::Div(
                                Box::new(calc2.clone()),
                                Box::new(calc1.clone()),
                            ));
                        }
                    }
                }
            }
        }
    }
}

fn has_solution(
    map1: &HashMap<DiceValue, Calculation>,
    map2: &HashMap<DiceValue, Calculation>,
    n: DiceValue,
) -> bool {
    for (r1, _) in map1 {
        for (r2, _) in map2 {
            if let Some(add) = r1.checked_add(*r2) {
                if add == n {
                    return true;
                }
            }

            if r1 >= r2 {
                if let Some(sub) = r1.checked_sub(*r2) {
                    if sub == n {
                        return true;
                    }
                }
            }

            if r2 >= r1 {
                if let Some(sub) = r2.checked_sub(*r1) {
                    if sub == n {
                        return true;
                    }
                }
            }

            if let Some(mult) = r1.checked_mul(*r2) {
                if mult == n {
                    return true;
                }
            }

            if *r2 > 0 {
                if *r1 % 10 == 0 && *r2 % 10 == 0 && *r1 > 0 {
                    continue;
                }
                if let Some(div) = r1.checked_div(*r2) {
                    if div * *r2 == *r1 {
                        if div as DiceValue == n {
                            return true;
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
                        if div as DiceValue == n {
                            return true;
                        }
                    }
                }
            }
        }
    }

    return false;
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dices = match self.num_dices {
            DiceAmount::Three => &self.dices[0..3],
            DiceAmount::Four => &self.dices,
        };

        let dices: Vec<_> = dices.iter().map(|&d| d.into()).collect();

        write!(
            f,
            "\n{}\n\n{}\n",
            Output::number_string(self.num.into()),
            Output::dices_string(&dices)
        )
    }
}
