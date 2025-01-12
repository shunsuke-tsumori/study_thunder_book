use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    fn new() -> Self {
        Self {
            x: 0usize,
            y: 0usize,
        }
    }
}

const H: usize = 3;
const W: usize = 4;
const END_TURN: i32 = 4;

pub struct MazeState {
    points: Vec<Vec<i32>>,
    turn: i32,
    pub character: Coord,
    pub game_score: i32,
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
}

fn random_action(state: &MazeState) -> usize {
    let legal_actions = state.legal_actions();
    let mut rng = StdRng::seed_from_u64(5);
    legal_actions[rng.gen_range(0..legal_actions.len())]
}

pub fn play_game(seed: u64) -> i32 {
    let mut state = MazeState::new(seed);
    println!("{}", state.to_string());
    while !state.is_done() {
        let action = random_action(&state);
        state.advance(action);
        println!("{}", state.to_string());
    }
    state.game_score
}
