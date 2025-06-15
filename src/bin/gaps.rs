use rayon::prelude::*;
use std::ops::Range;

use nums::{fastgame, game::Game};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        return;
    }

    let cube_count = match args[1].parse::<u8>() {
        Ok(3) => 3,
        Ok(4) => 4,
        _ => {
            eprintln!("Please provide either 3 or 4 cubes.");
            return;
        }
    };

    let time = std::time::Instant::now();
    match cube_count {
        3 => {
            println!("Finding distances for 3 cubes...");
            find_3_distances();
        }
        4 => {
            println!("Finding distances for 4 cubes...");
            find_4_distances();
        }
        _ => unreachable!(),
    }
    let elapsed = time.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
}

fn permutation_from_index(mut index: usize) -> [u8; 4] {
    let mut permutation = [0u8; 4];

    for i in 0..4 {
        permutation[i] = (index % 6) as u8 + 1; // 1 to 6
        index /= 6;
    }

    permutation
}

fn find_max_midpoint(solutions: &[u16], min: u16, max: u16) -> (u16, u16, u16) {
    if solutions.is_empty() {
        panic!("No solutions found");
    }

    let mut best_midpoint = 100;
    let mut best_distance = 0;
    let mut closest = 0;

    for pair in solutions.windows(2) {
        let (a, b) = (pair[0], pair[1]);

        let midpoint = a.saturating_add(b) / 2;
        let min_distance = b.saturating_sub(a) / 2;

        if min_distance > best_distance {
            best_distance = min_distance;
            best_midpoint = midpoint;
            closest = if midpoint.abs_diff(a) < midpoint.abs_diff(b) {
                a
            } else {
                b
            };
        }
    }

    // first
    let a = solutions.first().unwrap().clone();
    let b = min;
    let midpoint = a.saturating_add(b) / 2;
    let min_distance = b.saturating_sub(a) / 2;
    if min_distance > best_distance {
        best_distance = min_distance;
        best_midpoint = midpoint;
        closest = if midpoint.abs_diff(a) < midpoint.abs_diff(b) {
            a
        } else {
            b
        };
    }

    // last
    let a = solutions.last().unwrap().clone();
    let b = max;
    let midpoint = a.saturating_add(b) / 2;
    let min_distance = b.saturating_sub(a) / 2;
    if min_distance > best_distance {
        best_distance = min_distance;
        best_midpoint = midpoint;
        closest = if midpoint.abs_diff(a) < midpoint.abs_diff(b) {
            a
        } else {
            b
        };
    }

    (best_midpoint, best_distance, closest)
}

fn find_4_distances() -> Vec<Vec<u16>> {
    let permutation_count = 6 * 6 * 6 * 6; // 1296 permutations for 4 dice (1-6)
    let min_value = 100;
    let max_value = 1000;

    let mut distances: Vec<Vec<u16>> = Vec::with_capacity(permutation_count);
    for _ in 0..permutation_count {
        distances.push(Vec::new());
    }

    let default_game = fastgame::Game::new(fastgame::DiceAmount::Four);

    distances
        .par_iter_mut()
        .enumerate()
        .for_each_with(default_game, |game, (i, solutions)| {
            let permutation = permutation_from_index(i);
            let values: Range<u16> = min_value..max_value;

            for value in values {
                game.solve(value.into(), permutation);
                if game.solutions.len() > 0 {
                    solutions.push(value);
                }
            }

            let (mid, distance, closest) = find_max_midpoint(solutions, min_value, max_value);
            println!("Permutation: {permutation:?}, {mid} {distance} {closest}");
        });

    distances
}

fn find_3_distances() -> Vec<Vec<u16>> {
    let permutation_count = 6 * 6 * 6; // 216 permutations for 3 dice (1-6)
    let min_value = 0;
    let max_value = 100;
    let mut distances: Vec<Vec<u16>> = Vec::with_capacity(permutation_count);
    for _ in 0..permutation_count {
        distances.push(Vec::new());
    }

    let default_game = fastgame::Game::new(fastgame::DiceAmount::Three);

    distances
        .par_iter_mut()
        .enumerate()
        .for_each_with(default_game, |game, (i, solutions)| {
            let permutation = permutation_from_index(i);
            let values: Range<u16> = min_value..max_value;

            for value in values {
                game.solve(value.into(), permutation);
                if game.solutions.len() > 0 {
                    solutions.push(value);
                }
            }

            let (mid, distance, closest) = find_max_midpoint(solutions, min_value, max_value);
            println!(
                "Permutation: {:?}, {mid} {distance} {closest}",
                &permutation[0..3]
            );
        });

    distances
}
