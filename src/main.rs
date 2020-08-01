use std::thread;
use std::time;
use rand::random;


const LIVE: bool = true;
const DEAD: bool = false;

const LIVE_CHAR: &str = "\u{2588}\u{2588}";  // \u2588 OR \u2588
const DEAD_CHAR: &str = "\u{2591}\u{2591}";


pub struct BoardState {
    width: i32,
    height: i32,
    cells: Vec<Vec<bool>>,
}

impl BoardState {
    pub fn new(width: i32, height: i32) -> BoardState {
        let cells = vec![vec![DEAD; width as usize]; height as usize];

        return BoardState {width, height, cells};
        // let state = BoardState {width, height, cells};


        // return ref_state;
    }

    fn get_index(&self, x: i32, y: i32) -> bool {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            return self.cells[y as usize][x as usize];
        }
        return DEAD;
    }

    fn get_neighbors(&self, x: i32, y: i32) -> i32 {
        let mut neighbors = 0;

        for dy in [-1, 0, 1].iter().cloned() {
            for dx in [-1, 0, 1].iter().cloned() {
                // println!("deltas {:?}", (dx, dy));
                if dx == 0 && dy == 0 {
                    continue;
                }
                // println!("TEST {:?}", (x+dx, y+dy));
                if self.get_index(x+dx, y+dy) == LIVE {
                    neighbors += 1;
                }
            }
        }

        return neighbors;
    }

    pub fn gen_next(&mut self) {
        let mut next_state = self.cells.clone();
        let mut neighbors: i32;

        for y in 0..self.height {
            for x in 0..self.width {
                neighbors = self.get_neighbors(x, y);

                if self.cells[y as usize][x as usize] == LIVE {
                    if neighbors <= 1 {
                        next_state[y as usize][x as usize] = DEAD;
                    } else if neighbors >= 4 {
                        next_state[y as usize][x as usize] = DEAD;
                    } else {
                        next_state[y as usize][x as usize] = LIVE;
                    }
                } else {
                    if neighbors == 3 {
                        next_state[y as usize][x as usize] = LIVE;
                    } else {
                        next_state[y as usize][x as usize] = DEAD;
                    }
                }
            }
        }

        self.cells = next_state;
    }
}


fn init_state_empty(width: i32, height: i32) -> BoardState {
    let state = BoardState::new(width, height);
    // let ref_state: &BoardState = &state;

    return state;
}

fn init_state_random(width: i32, height: i32) -> BoardState {
    let mut state = BoardState::new(width, height);

    for y in 0..state.height {
        for x in 0..state.width {
            state.cells[y as usize][x as usize] = random();
        }
    }

    return state;
}

fn init_state_glider(width: i32, height: i32) -> BoardState {
    let mut state = init_state_empty(width, height);

    state.cells[2][0] = LIVE;
    state.cells[2][1] = LIVE;
    state.cells[2][2] = LIVE;
    state.cells[1][2] = LIVE;
    state.cells[0][1] = LIVE;

    return state;
}

fn draw(state: &BoardState) {
    println!("");
    println!("");
    println!("");
    println!("");

    for row in &state.cells {
        let mut line = String::from("");

        for cel in row {
            if *cel == LIVE {
                line.push_str(LIVE_CHAR);
            } else {
                line.push_str(DEAD_CHAR);
            }
        }
        println!("{}", line);
        line.clear();
    }
}

fn life(width: i32, height: i32, limit: i64, wait: u64, debug: bool) {
    // let mut state = init_state_empty(width, height);
    let mut state = init_state_random(width, height);
    // let mut state = init_state_glider(width, height);

    if debug {
        println!("Tick 1 !");
    } else {
        draw(&state);
    }

    for _ in 0..limit {
        thread::sleep(time::Duration::from_millis(wait));
        state.gen_next();

        if debug {
            println!("Tick !");
        } else {
            draw(&state);
        }
    }
}

fn main() {
    life(
        // width, height
        80, 38,
        // 1_000, 1_000,
        // max iter
        100_000,
        // sleep ms
        60,
        // debug, show time per tick insted of board
        false);
}
