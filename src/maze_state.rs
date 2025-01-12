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
const END_TURN: usize = 4;

pub struct MazeState {
    points: Vec<Vec<usize>>,
    turn: i32,
    pub character: Coord,
    pub game_score: i32,
}

impl MazeState {
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
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        Self {
            points,
            turn: 0,
            character: Coord { x, y },
            game_score: 0,
        }
    }
}
