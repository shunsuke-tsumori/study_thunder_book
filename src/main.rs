use crate::maze_state::MazeState;

mod maze_state;

fn main() {
    let mut maze_state = MazeState::new(12345);

    println!("{}", maze_state.to_string());

    let actions = maze_state.legal_actions();
    if !actions.is_empty() {
        maze_state.advance(actions[0]);
    }

    println!("{}", maze_state.to_string());
}