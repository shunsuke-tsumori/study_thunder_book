mod auto_move_maze_state;

use crate::auto_move_maze_state::{play_game, random_action, StringAIPair};

fn main() {
    let ai: StringAIPair = ("randomAction", random_action);
    play_game(ai, 0)
}
