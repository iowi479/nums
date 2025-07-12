use itertools::Itertools;
use nums::fastgame;
use rayon::prelude::*;
use std::io::Write;
use std::ops::Range;

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
    let mut gaps = match cube_count {
        3 => {
            println!("Finding distances for 3 cubes...");
            find_3_distances()
        }
        4 => {
            println!("Finding distances for 4 cubes...");
            find_4_distances()
        }
        _ => unreachable!(),
    };

    let elapsed = time.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);

    gaps.sort_by(|a, b| b.distance.cmp(&a.distance));

    let result_file = format!("gaps_{}.txt", cube_count);
    let mut file = std::fs::File::create(result_file).expect("Failed to create file");
    for gap in gaps.iter() {
        // let perm = permutation_from_index(gap.i);
        // let permutation = match cube_count {
        //     3 => &perm[..3],
        //     4 => &perm,
        //     _ => unreachable!(),
        // };

        let permutation = match cube_count {
            3 => &gap.permutation[..3],
            4 => &gap.permutation,
            _ => unreachable!(),
        };
        writeln!(
            &mut file,
            "{permutation:?} {} {} {} {:?}",
            gap.distance, gap.midpoint, gap.closest, gap.solutions
        )
        .expect("Failed to write to file");
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

struct GapResult {
    permutation: [u8; 4],
    solutions: Vec<u16>,
    midpoint: u16,
    distance: u16,
    closest: u16,
}

// fn find_4_distances() -> Vec<GapResult> {
//     let permutation_count = 6 * 6 * 6 * 6; // 1296 permutations for 4 dice (1-6)
//     let min_value = 100;
//     let max_value = 1000;
//
//     let mut distances: Vec<GapResult> = Vec::with_capacity(permutation_count);
//     for i in 0..permutation_count {
//         distances.push(GapResult {
//             i,
//             solutions: Vec::new(),
//             midpoint: 0,
//             distance: 0,
//             closest: 0,
//         });
//     }
//
//     distances.par_iter_mut().enumerate().for_each_init(
//         || fastgame::Game::new(fastgame::DiceAmount::Four),
//         |game, (i, gap_result)| {
//             let permutation = permutation_from_index(i);
//
//             game.solve_four_fast(min_value, max_value, permutation, &mut gap_result.solutions);
//
//             let (mid, distance, closest) =
//                 find_max_midpoint(&gap_result.solutions, min_value, max_value);
//             gap_result.midpoint = mid;
//             gap_result.distance = distance;
//             gap_result.closest = closest;
//
//             println!("Permutation: {permutation:?}, {mid} {distance} {closest}");
//         },
//     );
//
//     distances
// }
//
// fn find_3_distances() -> Vec<GapResult> {
//     let permutation_count = 6 * 6 * 6; // 216 permutations for 3 dice (1-6)
//     let min_value = 0;
//     let max_value = 100;
//
//     let mut permutations: Vec<GapResult> = Vec::with_capacity(permutation_count);
//     for i in 0..permutation_count {
//         permutations.push(GapResult {
//             i,
//             solutions: Vec::new(),
//             midpoint: 0,
//             distance: 0,
//             closest: 0,
//         });
//     }
//
//     permutations.par_iter_mut().enumerate().for_each_init(
//         || fastgame::Game::new(fastgame::DiceAmount::Three),
//         |game, (i, gap_result)| {
//             let permutation = permutation_from_index(i);
//
//             game.solve_three_fast(min_value, max_value, permutation, &mut gap_result.solutions);
//
//             let (mid, distance, closest) =
//                 find_max_midpoint(&gap_result.solutions, min_value, max_value);
//
//             gap_result.midpoint = mid;
//             gap_result.distance = distance;
//             gap_result.closest = closest;
//
//             println!(
//                 "Permutation: {:?} {i}, {mid} {distance} {closest}",
//                 &permutation[0..3]
//             );
//         },
//     );
//
//     permutations
// }

fn find_4_distances() -> Vec<GapResult> {
    // let min_value = 100;
    let min_value = 0;
    let max_value = 1000;

    let sides = 1..=6;
    let combos = sides.clone().combinations_with_replacement(4);
    let mut permutations: Vec<GapResult> = Vec::with_capacity(126);
    for perm in combos {
        permutations.push(GapResult {
            permutation: [perm[0] as u8, perm[1] as u8, perm[2] as u8, perm[3] as u8],
            solutions: Vec::new(),
            midpoint: 0,
            distance: 0,
            closest: 0,
        });
    }

    permutations.par_iter_mut().for_each_init(
        || fastgame::Game::new(fastgame::DiceAmount::Four),
        |game, gap_result| {
            game.solve_four_fast(
                min_value,
                max_value,
                gap_result.permutation,
                &mut gap_result.solutions,
            );

            let (mid, distance, closest) =
                find_max_midpoint(&gap_result.solutions, min_value, max_value);
            gap_result.midpoint = mid;
            gap_result.distance = distance;
            gap_result.closest = closest;

            println!(
                "Permutation: {:?}, {mid} {distance} {closest}",
                gap_result.permutation
            );
        },
    );

    permutations
}

fn find_3_distances() -> Vec<GapResult> {
    let min_value = 0;
    let max_value = 100;

    let sides = 1..=6;
    let combos = sides.clone().combinations_with_replacement(3);
    let mut permutations: Vec<GapResult> = Vec::with_capacity(56);
    for perm in combos {
        permutations.push(GapResult {
            permutation: [perm[0] as u8, perm[1] as u8, perm[2] as u8, 0],
            solutions: Vec::new(),
            midpoint: 0,
            distance: 0,
            closest: 0,
        });
    }

    permutations.par_iter_mut().for_each_init(
        || fastgame::Game::new(fastgame::DiceAmount::Three),
        |game, gap_result| {
            game.solve_three_fast(
                min_value,
                max_value,
                gap_result.permutation,
                &mut gap_result.solutions,
            );

            let (mid, distance, closest) =
                find_max_midpoint(&gap_result.solutions, min_value, max_value);

            gap_result.midpoint = mid;
            gap_result.distance = distance;
            gap_result.closest = closest;

            println!(
                "Permutation: {:?}, {mid} {distance} {closest}",
                &gap_result.permutation[0..3]
            );
        },
    );

    permutations
}
