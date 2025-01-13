use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Clone, Eq, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

const H: usize = 30;
const W: usize = 30;
const END_TURN: i32 = 100;

#[derive(Clone, Eq, PartialEq)]
pub struct MazeState {
    points: Vec<Vec<i32>>,
    turn: i32,
    pub character: Coord,
    pub game_score: i32,
    pub evaluated_score: i32,
    pub first_action: isize,
}

impl MazeState {
    const DX: [isize; 4] = [1, -1, 0, 0];
    const DY: [isize; 4] = [0, 0, 1, -1];

    pub fn new(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let x = rng.gen_range(0..H);
        let y = rng.gen_range(0..W);

        let points = (0..H)
            .map(|row_idx| {
                (0..W)
                    .map(|col_idx| {
                        if row_idx == x && col_idx == y {
                            0
                        } else {
                            rng.gen_range(0..10)
                        }
                    })
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        Self {
            points,
            turn: 0,
            character: Coord { x, y },
            game_score: 0,
            evaluated_score: 0,
            first_action: -1,
        }
    }

    pub fn is_done(&self) -> bool {
        self.turn == END_TURN
    }

    fn is_in_board(&self, x: isize, y: isize) -> bool {
        0 <= x && x < H as isize && 0 <= y && y < W as isize
    }

    pub(crate) fn legal_actions(&self) -> Vec<usize> {
        let mut actions = vec![];
        for action in 0..4 {
            let nx = self.character.x as isize + Self::DX[action];
            let ny = self.character.y as isize + Self::DY[action];
            if self.is_in_board(nx, ny) {
                actions.push(action);
            }
        }
        actions
    }

    pub fn advance(&mut self, action: usize) {
        self.character.x = (self.character.x as isize + Self::DX[action]) as usize;
        self.character.y = (self.character.y as isize + Self::DY[action]) as usize;
        let point = &mut self.points[self.character.x][self.character.y];
        self.game_score += *point;
        *point = 0;
        self.turn += 1;
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str(&format!(
            "Turn: {}, Score: {}\n",
            self.turn, self.game_score
        ));

        for row in 0..H {
            for col in 0..W {
                if row == self.character.x && col == self.character.y {
                    result.push('@');
                } else if self.points[row][col] > 0 {
                    let val = self.points[row][col];
                    result.push_str(&val.to_string());
                } else {
                    result.push('.');
                }
                result.push(' ');
            }
            result.push('\n');
        }

        result
    }

    pub fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score
    }
}

#[allow(dead_code)]
pub fn random_action(state: &MazeState) -> usize {
    let legal_actions = state.legal_actions();
    let mut rng = rand::thread_rng();
    legal_actions[rng.gen_range(0..legal_actions.len())]
}

pub fn greedy_action(state: &MazeState) -> usize {
    let legal_actions = state.legal_actions();
    let mut max_score = i32::MIN;
    let mut best_action = legal_actions[0];
    for action in legal_actions {
        let mut current_state = state.clone();
        current_state.advance(action);
        current_state.evaluate_score();
        if current_state.evaluated_score > max_score {
            max_score = current_state.evaluated_score;
            best_action = action;
        }
    }

    best_action
}

pub fn play_game<F>(seed: u64, policy: F, verbose: bool) -> i32
where
    F: Fn(&MazeState) -> usize,
{
    let mut state = MazeState::new(seed);
    if verbose {
        println!("{}", state.to_string());
    }
    while !state.is_done() {
        let action = policy(&state);
        state.advance(action);
        if verbose {
            println!("{}", state.to_string());
        }
    }
    state.game_score
}
