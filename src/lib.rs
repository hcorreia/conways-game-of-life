use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use rand::random;
use libc;
use std::ffi::CString;
// use std::ffi::c_void;


const LIVE: u8 = 1;
const DEAD: u8 = 0;

const LIVE_CHAR: &str = "\u{2588}\u{2588}";  // \u2588 OR \u2588
const DEAD_CHAR: &str = "\u{2591}\u{2591}";


#[repr(C)]
#[derive(Debug)]
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

    pub fn new_random(width: i32, height: i32) -> BoardState {
        let mut cells = vec![DEAD; (width*height) as usize];

        for i in 0..(height*width) {
            cells[i as usize] = random::<bool>() as u8;
        }

        return BoardState {width, height, cells};
    }

    pub fn new_glider(width: i32, height: i32) -> BoardState {
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
    state: Arc<BoardState>,
}

impl Life {
    pub fn new(width: i32, height: i32) -> Life {
        let state = Arc::new(BoardState::new(width, height));
        return Life {state};
    }

    pub fn new_random(width: i32, height: i32) -> Life {
        let state = Arc::new(BoardState::new_random(width, height));
        return Life {state};
    }

    pub fn new_glider(width: i32, height: i32) -> Life {
        let state = Arc::new(BoardState::new_glider(width, height));
        return Life {state};
    }

    pub fn tick(&mut self) {
        // let mut next_state = self.cells.clone();
        let next_cells = vec![DEAD; (self.state.width*self.state.height) as usize];
        // let next_state = BoardState::new(self.state.width, self.state.height);
        // let mut neighbors: u8;
        // let mut x;
        // let mut y;

        let mut workers = Vec::new();

        // let state = Arc::clone(&self.state);
        let c_lock = Arc::new(Mutex::new(next_cells));

        // let width = self.state.width;
        // let height = self.state.height;

        // let state_arc = Arc::new(Mutex::new(&self));
        let cpus: usize = 2;
        let slice_size = ((self.state.width*self.state.height) as usize)/cpus;
        // let slices = next_cells.slice_at(slice_size);

        for n in 0..cpus {
            let state = Arc::clone(&self.state);
            let c_lock2 = c_lock.clone();

            // let &mut cells_slice = next_cells[(slice_size*n)..(slice_size*n+slice_size-1)];
            // let cells_slice = Arc::new(cells_slice);
            // println!("{:?}", (slice_size*n, slice_size*n+slice_size-1));

            let from = (slice_size*n) as i32;
            let to = (slice_size*n+slice_size) as i32;

            // println!("{:?}", (from, to));

            workers.push(thread::spawn(move || {
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
            }));
        }


        for worker in workers {
            let _ = worker.join();
        }

        // for worker in &mut self.workers {
        //     // let _ = worker.join();

        //     if let Some(thread) = worker.take() {
        //         thread.join().unwrap();
        //     }
        // }

let cells = c_lock.lock().unwrap();
let width = self.state.width;
let height = self.state.height;
let next_state = BoardState{width, height, cells: cells.to_vec()};
// println!("{:?}", next_state.width);

        // self.state = Arc::new(next_state);
        // let next_state = c_lock.lock().unwrap();
        self.state = Arc::new(next_state);

        // return next_state;
    }
}


fn init_state_empty(width: i32, height: i32) -> Life {
    return Life::new(width, height);
}


fn init_state_random(width: i32, height: i32) -> Life {
    return Life::new_random(width, height);
}


fn init_state_glider(width: i32, height: i32) -> Life {
    return Life::new_glider(width, height);
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


pub fn life(width: i32, height: i32, limit: i64, wait: u64, debug: bool) {
    let sleep_time = time::Duration::from_millis(wait);
    let mut now;
    let mut game;

    if debug {
        now = time::SystemTime::now();
        // game = init_state_empty(width, height);
        game = init_state_random(width, height);
        // game = init_state_glider(width, height);
        println!("Tick 1 ! {:?}", now.elapsed());
    } else {
        // game = init_state_empty(width, height);
        game = init_state_random(width, height);
        // game = init_state_glider(width, height);
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


#[no_mangle] // *mut libc::c_void
pub extern "C" fn init_state_random_2(width: i32, height: i32) -> *mut Life {
    println!("{:?}, {:?}", width, height);
    let game = Life::new_random(width, height);
    println!("{:?}, {:?}", game.state.width, game.state.height);
    // for i in 0..(game.height*game.width) {
    //     game.cells[i as usize] = random::<bool>() as u8;
    // }

    let raw = Box::into_raw(Box::new(game));
    println!("return ptr {:?}", raw);
    return raw;
    // return Box::into_raw(Box::new(game)) as *mut libc::c_void;
}


#[no_mangle]
pub extern "C" fn next_state(game_ptr: *mut libc::c_void) -> *mut libc::c_char {
    println!("next_state ptr {:?}", game_ptr);
    // let game: &mut Life = unsafe { &mut *(game_ptr as *mut Life) };
    // let mut game = unsafe { Box::from_raw(game_ptr as *mut Life) };
    let mut game = unsafe { Box::from_raw(game_ptr as *mut Life) };

    let mut buff = String::from("");
    println!("{:?}", (game.state.width, game.state.height));
    game.tick();

    for (i, cell) in game.state.cells.iter().enumerate() {

        if *cell == LIVE {
            buff.push_str(LIVE_CHAR);
        } else {
            buff.push_str(DEAD_CHAR);
        }

        if  (i as i32  + 1) % game.state.width == 0 {
            buff.push_str("\n");
        }
    }

    let raw = Box::into_raw(Box::new(game));
    println!("return ptr {:?}", raw);

    let c_str_song = CString::new(buff).unwrap();
    return c_str_song.into_raw();
}


#[no_mangle]
pub extern "C" fn free_char_p(s: *mut libc::c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}


#[no_mangle]
pub extern "C" fn free_void_p(ptr: *mut libc::c_void) {
    println!("free ptr {:?}", ptr);
    unsafe {
        if ptr.is_null() {
            return;
        }
        Box::from_raw(ptr)
    };
}
