use crate::maze_state::MazeState;
use std::collections::BinaryHeap;

const BEAM_WIDTH: usize = 1;
const BEAM_DEPTH: usize = 7;
const BEAM_NUMBER: usize = 7;

pub fn chokudai_search_action(state: &MazeState) -> usize {
    let mut beam: Vec<BinaryHeap<MazeState>> = vec![BinaryHeap::new(); BEAM_DEPTH + 1];
    beam[0].push(state.clone());

    for _ in 0..BEAM_NUMBER {
        for depth in 0..BEAM_DEPTH {
            let (left, right) = beam.split_at_mut(depth + 1);
            let current_beam = &mut left[depth];
            let next_beam = &mut right[0];

            for _ in 0..BEAM_WIDTH {
                if current_beam.is_empty() {
                    break;
                }
                let current_state = current_beam.pop().unwrap();

                if current_state.is_done() {
                    break;
                }

                let legal_actions = current_state.legal_actions();
                for &action in &legal_actions {
                    let mut next_state = current_state.clone();
                    next_state.advance(action);
                    next_state.evaluate_score();
                    if depth == 0 {
                        next_state.first_action = action as isize;
                    }
                    next_beam.push(next_state);
                }
            }
        }
    }

    for depth in (0..=BEAM_DEPTH).rev() {
        if let Some(top_state) = beam[depth].peek() {
            return top_state.first_action as usize;
        }
    }
    state.legal_actions()[0]
}
