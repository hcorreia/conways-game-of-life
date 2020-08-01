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
    cells: Vec<u8>,
}

impl BoardState {
    pub fn new(width: i32, height: i32) -> BoardState {
        let cells = vec![DEAD; (width*height) as usize];

        return BoardState {width, height, cells};
    }

    fn get_index(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            // println!("deltas {:?}", (x, y));
            return self.cells[(y*self.width + x) as usize];
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
        let mut next_state = vec![DEAD; (self.width*self.height) as usize];
        let mut neighbors: u8;
        let mut x;
        let mut y;

        for i in 0..(self.height*self.width) {
            x = i % self.width;
            y = i / self.width;

            neighbors = self.get_neighbors(x, y);

            if self.cells[i as usize] == LIVE {
                if neighbors <= 1 {
                    next_state[i as usize] = DEAD;
                } else if neighbors >= 4 {
                    next_state[i as usize] = DEAD;
                } else {
                    next_state[i as usize] = LIVE;
                }
            } else {
                if neighbors == 3 {
                    next_state[i as usize] = LIVE;
                } else {
                    next_state[i as usize] = DEAD;
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

    for i in 0..(state.height*state.width) {
        state.cells[i as usize] = random::<bool>() as u8;
    }

    return state;
}


fn init_state_glider(width: i32, height: i32) -> BoardState {
    let mut state = init_state_empty(width, height);

    state.cells[(2*width + 0) as usize] = LIVE;
    state.cells[(2*width + 1) as usize] = LIVE;
    state.cells[(2*width + 2) as usize] = LIVE;
    state.cells[(1*width + 2) as usize] = LIVE;
    state.cells[(0*width + 1) as usize] = LIVE;

    return state;
}


fn draw(state: &BoardState) {
    let mut line = String::from("");

    println!("");
    println!("");
    println!("");
    println!("");

    for (i, cell) in state.cells.iter().enumerate() {

        if *cell == LIVE {
            line.push_str(LIVE_CHAR);
        } else {
            line.push_str(DEAD_CHAR);
        }

        if  (i as i32  + 1) % state.width == 0 {
            println!("{}", line);
            line.clear();
        }
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
        // state = init_state_glider(width, height);
        println!("Tick 1 ! {:?}", now.elapsed());
    } else {
        state = init_state_random(width, height);
        // state = init_state_glider(width, height);
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
        // 5, 10,
        2_000, 2_000,
        // max iter
        10,
        // sleep ms
        60,
        // debug, show time per tick insted of board
        true);
}
