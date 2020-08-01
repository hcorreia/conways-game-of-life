use std::thread;
use std::time;
use rand::random;


const LIVE: u8 = 1;
const DEAD: u8 = 0;

const LIVE_CHAR: &str = "\u{2588}\u{2588}";  // \u2588 OR \u2588
const DEAD_CHAR: &str = "\u{2591}\u{2591}";


pub struct BoardState {
    width: i32,
    height: i32,
    cells: Vec<Vec<u8>>,
}

impl BoardState {
    pub fn new(width: i32, height: i32) -> BoardState {
        let cells = vec![vec![DEAD; width as usize]; height as usize];

        return BoardState {width, height, cells};
        // let state = BoardState {width, height, cells};


        // return ref_state;
    }

    fn get_index(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            // println!("deltas {:?}", (x, y));
            return self.cells[y as usize][x as usize];
        }
        return DEAD;
    }

    fn get_neighbors(&self, x: i32, y: i32) -> u8 {
        return
            self.get_index(x-1, y-1) +
            self.get_index(x  , y-1) +
            self.get_index(x+1, y-1) +
            self.get_index(x+1, y  ) +
            self.get_index(x+1, y+1) +
            self.get_index(x  , y+1) +
            self.get_index(x-1, y+1) +
            self.get_index(x-1, y  );
    }

    pub fn gen_next(&mut self) {
        // let mut next_state = self.cells.clone();
        let mut next_state = vec![vec![DEAD; self.width as usize]; self.height as usize];
        let mut neighbors: u8;

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
            state.cells[y as usize][x as usize] = random::<bool>() as u8;
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
    let sleep_time = time::Duration::from_millis(wait);
    let mut now;
    let mut state;
    // let mut state = init_state_empty(width, height);
    // let mut state = init_state_random(width, height);
    // let mut state = init_state_glider(width, height);

    if debug {
        now = time::SystemTime::now();
        state = init_state_random(width, height);
        println!("Tick 1 ! {:?}", now.elapsed());
    } else {
        state = init_state_random(width, height);
        draw(&state);
    }

    for _ in 0..limit {
        thread::sleep(sleep_time);

        if debug {
            now = time::SystemTime::now();
            state.gen_next();
            println!("Tick ! {:?}", now.elapsed());
        } else {
            state.gen_next();
            draw(&state);
        }
    }
}

fn main() {
    life(
        // width, height
        // 80, 38,
        2_000, 2_000,
        // max iter
        10,
        // sleep ms
        60,
        // debug, show time per tick insted of board
        true);
}
