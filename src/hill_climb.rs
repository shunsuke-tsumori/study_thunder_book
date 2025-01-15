use crate::auto_move_maze_state::AutoMoveMazeState;

const NUMBER: i32 = 10000;

pub fn hill_climb(state: AutoMoveMazeState, seed: u64) -> AutoMoveMazeState {
    let mut now_state = state.clone();
    now_state.init(seed);
    let mut best_score = now_state.get_score(false);

    for _ in 0..NUMBER {
        let mut next_state = now_state.clone();
        next_state.transition();
        let next_score = next_state.get_score(false);

        if next_score > best_score {
            best_score = next_score;
            now_state = next_state;
        }
    }

    now_state
}
