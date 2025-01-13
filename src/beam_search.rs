use crate::maze_state::MazeState;
use crate::time_keeper::TimeKeeper;
use std::collections::BinaryHeap;

const BEAM_WIDTH: usize = 4;
const BEAM_DEPTH: usize = 10;
const TIME_THRESHOLD_MS: i64 = 10;

pub fn beam_search_action(state: &MazeState) -> usize {
    let mut current_beam = BinaryHeap::new();
    let mut best_state: MazeState = state.clone();
    let time_keeper = TimeKeeper::new(TIME_THRESHOLD_MS);

    current_beam.push(state.clone());
    for depth in 0..BEAM_DEPTH {
        let mut next_beam = BinaryHeap::new();
        for _ in 0..BEAM_WIDTH {
            if time_keeper.is_time_over() {
                return best_state.first_action as usize;
            }
            let current_state = if let Some(s) = current_beam.pop() {
                s
            } else {
                break;
            };
            let legal_actions = current_state.legal_actions();
            for action in legal_actions {
                let mut next_state = current_state.clone();
                next_state.advance(action);
                next_state.evaluate_score();
                if depth == 0 {
                    next_state.first_action = action as isize;
                }
                next_beam.push(next_state);
            }
        }

        current_beam = next_beam;
        if current_beam.is_empty() {
            break;
        }

        if let Some(top_state) = current_beam.peek() {
            best_state = top_state.clone();
        }
        if best_state.is_done() {
            break;
        }
    }

    best_state.first_action as usize
}
