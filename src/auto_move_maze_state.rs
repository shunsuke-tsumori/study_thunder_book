use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Clone, Eq, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

const H: usize = 5;
const W: usize = 5;
const END_TURN: i32 = 5;
const CHARACTER_N: usize = 3;

#[derive(Clone, Eq, PartialEq)]
pub struct AutoMoveMazeState {
    points: Vec<Vec<i32>>,
    turn: i32,
    pub characters: Vec<Coord>,
    pub evaluated_score: i32,
    pub game_score: i64,
}

impl AutoMoveMazeState {
    const DX: [isize; 4] = [1, -1, 0, 0];
    const DY: [isize; 4] = [0, 0, 1, -1];

    pub fn new(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut points = vec![vec![0; W]; H];
        for y in 0..H {
            for x in 0..W {
                points[y][x] = rng.gen_range(1..=9);
            }
        }
        Self {
            points,
            turn: 0,
            characters: vec![Coord { x: 0, y: 0 }; CHARACTER_N],
            evaluated_score: 0,
            game_score: 0,
        }
    }

    pub fn set_character(&mut self, character_id: usize, y: usize, x: usize) {
        if character_id < self.characters.len() {
            self.characters[character_id].y = y;
            self.characters[character_id].x = x;
        }
    }

    pub fn is_done(&self) -> bool {
        self.turn == END_TURN
    }

    fn move_player(&mut self, character_id: usize) {
        let (current_y, current_x) = {
            let ch = &self.characters[character_id];
            (ch.y, ch.x)
        };
        let mut best_point = i32::MIN;
        let mut best_action_index = 0;

        for action in 0..4 {
            let ny = current_y as isize + Self::DY[action];
            let nx = current_x as isize + Self::DX[action];
            if ny >= 0 && ny < H as isize && nx >= 0 && nx < W as isize {
                let point = self.points[ny as usize][nx as usize];
                if point > best_point {
                    best_point = point;
                    best_action_index = action;
                }
            }
        }
        self.characters[character_id].y = (current_y as isize + Self::DY[best_action_index]) as usize;
        self.characters[character_id].x = (current_x as isize + Self::DX[best_action_index]) as usize;
    }

    pub fn advance(&mut self) {
        for character_id in 0..CHARACTER_N {
            self.move_player(character_id);
        }
        for ch in &self.characters {
            let point = &mut self.points[ch.y][ch.x];
            self.game_score += *point as i64;
            *point = 0;
        }
        self.turn += 1;
    }

    pub fn to_string(&self) -> String {
        let mut result = format!("turn:\t{}\nscore:\t{}\n", self.turn, self.game_score);
        for y in 0..H {
            for x in 0..W {
                let mut found_char = false;
                for ch in &self.characters {
                    if ch.y == y && ch.x == x {
                        result.push('@');
                        found_char = true;
                        break;
                    }
                }
                if !found_char {
                    let p = self.points[y][x];
                    if p > 0 {
                        result.push_str(&p.to_string());
                    } else {
                        result.push('.');
                    }
                }
            }
            result.push('\n');
        }
        result
    }

    pub fn get_score(&self, is_print: bool) -> i64 {
        let mut tmp = self.clone();
        for ch in &tmp.characters {
            tmp.points[ch.y][ch.x] = 0;
        }
        while !tmp.is_done() {
            tmp.advance();
            if is_print {
                println!("{}", tmp.to_string());
            }
        }
        tmp.game_score
    }
}

pub fn random_action(mut state: AutoMoveMazeState, seed: u64) -> AutoMoveMazeState {
    let mut rng = StdRng::seed_from_u64(seed);
    for character_id in 0..CHARACTER_N {
        let y = rng.gen_range(0..H);
        let x = rng.gen_range(0..W);
        state.set_character(character_id, y, x);
    }
    state
}

type AIFunction = fn(AutoMoveMazeState, u64) -> AutoMoveMazeState;
pub type StringAIPair = (&'static str, AIFunction);

pub fn play_game(ai: StringAIPair, seed: u64) {
    let mut state = AutoMoveMazeState::new(seed);
    state = (ai.1)(state, seed);

    println!("{}", state.to_string());
    let score = state.get_score(true);
    println!("Score of {}: {}", ai.0, score);
}
