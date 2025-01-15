use crate::auto_move_maze_state::{AutoMoveMazeState, StringAIPair};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn test_ai_score(ai: StringAIPair, game_number: i32) {
    let mut mt_for_construct = StdRng::seed_from_u64(0);
    let mut score_sum = 0.0;

    for _ in 0..game_number {
        let seed = mt_for_construct.gen::<u64>();
        let mut state = AutoMoveMazeState::new(seed);
        state = (ai.1)(state, seed);
        let score = state.get_score(false);
        score_sum += score as f64;
    }

    let score_mean = score_sum / game_number as f64;
    println!("Score of {}:\t{}", ai.0, score_mean);
}
