use crate::maze_state::{greedy_action, play_game, random_action};
use crate::beam_search::beam_search_action;

mod maze_state;
mod beam_search;

fn main() {
    let trials = 100;
    let mut sum_random = 0;
    let mut sum_greedy = 0;
    let mut sum_beam = 0;

    for i in 0..trials {
        let score = play_game(i as u64, random_action);
        sum_random += score;
    }
    let avg_random = sum_random as f64 / trials as f64;

    for i in 0..trials {
        let score = play_game(i as u64, greedy_action);
        sum_greedy += score;
    }
    let avg_greedy = sum_greedy as f64 / trials as f64;

    for i in 0..trials {
        let score = play_game(i as u64, beam_search_action);
        sum_beam += score;
    }
    let avg_beam = sum_beam as f64 / trials as f64;

    println!(
        "Random Action Average Score ({} trials) = {}",
        trials, avg_random
    );
    println!(
        "Greedy Action Average Score ({} trials) = {}",
        trials, avg_greedy
    );
    println!(
        "Beam Search Action Average Score ({} trials) = {}",
        trials, avg_beam
    );
}
