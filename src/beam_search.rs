use crate::maze_state::MazeState;
use std::collections::BinaryHeap;

const BEAM_WIDTH: usize = 2;
const BEAM_DEPTH: usize = 2;

impl Ord for MazeState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
    }
}

impl PartialOrd for MazeState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn beam_search_action(state: &MazeState) -> usize {
    let mut current_beam = BinaryHeap::new();
    let mut best_state: MazeState = state.clone();

    current_beam.push(state.clone());
    for depth in 0..BEAM_DEPTH {
        let mut next_beam = BinaryHeap::new();
        for _ in 0..BEAM_WIDTH {
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
