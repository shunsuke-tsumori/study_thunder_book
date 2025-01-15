mod auto_move_maze_state;
mod hill_climb;

use crate::auto_move_maze_state::{play_game, random_action, StringAIPair};
use crate::hill_climb::hill_climb;

fn main() {
    let ai: StringAIPair = ("randomAction", random_action);
    play_game(ai, 0);

    let ai2: StringAIPair = ("hillClimbAction", hill_climb);
    play_game(ai2, 0);
}
