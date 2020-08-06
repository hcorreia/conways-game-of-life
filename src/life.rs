use rand::random;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use threadpool::ThreadPool;


pub const LIVE: u8 = 1;
pub const DEAD: u8 = 0;

pub const LIVE_CHAR: &str = "\u{2588}\u{2588}";  // \u2588 OR \u2588
pub const DEAD_CHAR: &str = "\u{2591}\u{2591}";


pub enum Shape {
    Empty,
    Random,
    Glider,
}


#[repr(C)]
#[derive(Debug)]
pub struct BoardState {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<u8>,
}

impl BoardState {
    pub fn new(width: i32, height: i32, init: Shape) -> BoardState {
        return match init {
            Shape::Empty => BoardState::new_empty(width, height),
            Shape::Random => BoardState::new_random(width, height),
            Shape::Glider => BoardState::new_glider(width, height),
        }
    }

    fn new_empty(width: i32, height: i32) -> BoardState {
        let cells = vec![DEAD; (width*height) as usize];
        return BoardState {width, height, cells};
    }

    fn new_random(width: i32, height: i32) -> BoardState {
        let mut cells = vec![DEAD; (width*height) as usize];

        for i in 0..(height*width) {
            cells[i as usize] = random::<bool>() as u8;
        }

        return BoardState {width, height, cells};
    }

    fn new_glider(width: i32, height: i32) -> BoardState {
        let mut cells = vec![DEAD; (width*height) as usize];

        cells[(2*width + 0) as usize] = LIVE;
        cells[(2*width + 1) as usize] = LIVE;
        cells[(2*width + 2) as usize] = LIVE;
        cells[(1*width + 2) as usize] = LIVE;
        cells[(0*width + 1) as usize] = LIVE;

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
}


#[repr(C)]
#[derive(Debug)]
pub struct Life {
    pub state: Arc<BoardState>,
    pool: ThreadPool,
}

impl Life {
    pub fn new(width: i32, height: i32, init: Shape, n_workers: usize) -> Life {
        let state = Arc::new(BoardState::new(width, height, init));
        let pool = ThreadPool::new(n_workers);
        return Life {state, pool};
    }

    pub fn tick(&mut self) {
        // let mut next_state = self.cells.clone();
        let next_cells = vec![DEAD; (self.state.width*self.state.height) as usize];
        // let next_state = BoardState::new(self.state.width, self.state.height);
        // let mut neighbors: u8;
        // let mut x;
        // let mut y;

        // let mut workers = Vec::new();

        // let state = Arc::clone(&self.state);
        let c_lock = Arc::new(Mutex::new(next_cells));

        // let width = self.state.width;
        // let height = self.state.height;

        // let state_arc = Arc::new(Mutex::new(&self));
        let cpus: usize = self.pool.max_count();
        // let chunk_size = (self.state.width*self.state.height) as usize/cpus;
        let slice_size = ((self.state.width*self.state.height) as usize)/cpus;
        // let slices = next_cells.slice_at(slice_size);

        for n in 0..cpus {
            let state = Arc::clone(&self.state);
            let c_lock2 = c_lock.clone();

            // TODO: fix
            let from = (slice_size*n) as i32;
            let to = (slice_size*n+slice_size) as i32;

            self.pool.execute(move || {
                // println!("{:?}", (width, height));
                // println!("{:?}", state);
                for i in from..to {

                    let x = i % state.width;
                    let y = i / state.width;

                    let neighbors = state.get_neighbors(x, y);

                    if state.cells[i as usize] == LIVE {
                        if neighbors <= 1 {
                            // c_lock2.lock().unwrap()[i as usize] = DEAD;
                        } else if neighbors >= 4 {
                            // c_lock2.lock().unwrap()[i as usize] = DEAD;
                        } else {
                            c_lock2.lock().unwrap()[i as usize] = LIVE;
                        }
                    } else {
                        if neighbors == 3 {
                            c_lock2.lock().unwrap()[i as usize] = LIVE;
                        } else {
                            // c_lock2.lock().unwrap()[i as usize] = DEAD;
                        }
                    }
                }

                // println!("DONE worker{:?}", n);
            });
        }

        self.pool.join();
// println!("B{:?}", next_cells);
// thread::sleep(time::Duration::from_millis(1000));

        let cells = c_lock.lock().unwrap();
        let width = self.state.width;
        let height = self.state.height;
        let next_state = BoardState{width, height, cells: cells.to_vec()};

        self.state = Arc::new(next_state);
    }
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


pub fn start_life(
    width: i32, height: i32,
    init: Shape,
    n_workers: usize,
    limit: i64,
    wait: u64, debug: bool,
) {
    let sleep_time = time::Duration::from_millis(wait);
    let mut now;
    let mut game;

    if debug {
        now = time::SystemTime::now();
        game = Life::new(width, height, init, n_workers);
        println!("Tick 1 ! {:?}", now.elapsed());
    } else {
        game = Life::new(width, height, init, n_workers);
        draw(&game.state);
    }

    for _ in 0..limit {
        thread::sleep(sleep_time);

        if debug {
            now = time::SystemTime::now();
            game.tick();
            println!("Tick ! {:?}", now.elapsed());
        } else {
            game.tick();
            draw(&game.state);
        }
    }
}
