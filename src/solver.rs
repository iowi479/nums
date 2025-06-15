use std::ops::Range;

use rayon::prelude::*;

use crate::game::{DiceAmount, Game};

pub struct Solver {
    pub best_distance: Vec<Vec<u16>>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            best_distance: Vec::with_capacity(1296),
        }
    }

    fn find_4_distances(&mut self) -> Vec<Vec<u16>> {
        let mut distances: Vec<Vec<u16>> = Vec::with_capacity(1296);
        for _ in 0..1296 {
            distances.push(Vec::new());
        }

        self.best_distance
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, solutions)| {
                let permutation = permutation_from_index(i);
                let values: Range<u16> = 100..1000;

                for value in values {
                    let mut game = Game::of(
                        DiceAmount::Four,
                        value as u64,
                        [
                            permutation[0] as u64,
                            permutation[1] as u64,
                            permutation[2] as u64,
                            permutation[3] as u64,
                        ],
                    );
                    game.solve();
                    if game.solutions.len() > 0 {
                        solutions.push(value);
                    }
                }

                let (mid, distance) = find_max_midpoint(solutions, 100, 1000);
                println!("Permutation: {permutation:?}, {mid} {distance}");
            });

        distances
    }
}

fn permutation_from_index(mut index: usize) -> [u8; 4] {
    let mut permutation = [0u8; 4];

    for i in 0..4 {
        permutation[i] = (index % 6) as u8 + 1; // 1 to 6
        index /= 6;
    }

    permutation
}

fn find_max_midpoint(solutions: &[u16], min: u16, max: u16) -> (u16, u16) {
    if solutions.is_empty() {
        panic!("No solutions found");
    }

    let mut best_midpoint = 100;
    let mut best_distance = 0;

    for pair in solutions.windows(2) {
        let (a, b) = (pair[0], pair[1]);

        let midpoint = (a + b) / 2;
        let min_distance = (b - a) / 2;

        if min_distance > best_distance {
            best_distance = min_distance;
            best_midpoint = midpoint;
        }
    }

    // first
    let midpoint = (solutions[0] + min) / 2;
    let min_distance = (solutions[0] - min) / 2;
    if min_distance > best_distance {
        best_distance = min_distance;
        best_midpoint = midpoint;
    }

    // last
    let midpoint = (solutions.last().unwrap() + max) / 2;
    let min_distance = (max - solutions.last().unwrap()) / 2;
    if min_distance > best_distance {
        best_distance = min_distance;
        best_midpoint = midpoint;
    }

    (best_midpoint, best_distance)
}
