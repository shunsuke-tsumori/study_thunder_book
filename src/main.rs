mod auto_move_maze_state;
mod hill_climb;
mod simulated_annealing;
mod test_score;

use crate::auto_move_maze_state::{random_action, StringAIPair};
use crate::hill_climb::hill_climb;
use crate::simulated_annealing::simulated_annealing;
use crate::test_score::test_ai_score;

fn main() {
    let ai: StringAIPair = ("randomAction", random_action);
    test_ai_score(ai, 100);

    let ai2: StringAIPair = ("hillClimbAction", hill_climb);
    test_ai_score(ai2, 100);

    let ai3: StringAIPair = ("simulatedAnnealingAction", simulated_annealing);
    test_ai_score(ai3, 100);
}
