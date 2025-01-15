use crate::auto_move_maze_state::AutoMoveMazeState;
use rand::{thread_rng, Rng};

const NUMBER: i32 = 10000;
const START_TEMP: f64 = 500.0;
const END_TEMP: f64 = 10.0;

pub fn simulated_annealing(state: AutoMoveMazeState, seed: u64) -> AutoMoveMazeState {
    let mut now_state = state.clone();
    now_state.init(seed);
    let mut best_score = now_state.get_score(false);
    let mut now_score = best_score;
    let mut best_state = now_state.clone();

    let mut rng = thread_rng();

    for i in 0..NUMBER {
        let mut next_state = now_state.clone();
        next_state.transition();
        let next_score = next_state.get_score(false);
        let temp = START_TEMP + (END_TEMP - START_TEMP) * (i as f64 / NUMBER as f64);
        let probability = f64::exp(((next_score as f64) - (now_score as f64)) / temp);
        let is_force_next = probability > rng.gen_range(0.0..1.0);

        if next_score > now_score || is_force_next {
            now_score = next_score;
            now_state = next_state.clone();
        }

        if next_score > best_score {
            best_score = next_score;
            best_state = next_state;
        }
    }
    best_state
}
