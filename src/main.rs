use crate::maze_state::MazeState;

mod maze_state;

fn main() {
    let maze_state = MazeState::new(12345);
    println!("Character = ({}, {})", maze_state.character.x, maze_state.character.y);
}
