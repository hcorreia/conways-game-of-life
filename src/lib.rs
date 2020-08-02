use std::thread;
use std::time;
use rand::random;
use libc;
use std::ffi::CString;
use std::ffi::c_void;


const LIVE: u8 = 1;
const DEAD: u8 = 0;

const LIVE_CHAR: &str = "\u{2588}\u{2588}";  // \u2588 OR \u2588
const DEAD_CHAR: &str = "\u{2591}\u{2591}";


#[repr(C)]
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


pub fn life(width: i32, height: i32, limit: i64, wait: u64, debug: bool) {
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


#[no_mangle] // *mut libc::c_void
pub extern "C" fn init_state_random_2(width: i32, height: i32) -> *mut BoardState {
    println!("{:?}, {:?}", width, height);
    let mut state = BoardState::new(width, height);
    println!("{:?}, {:?}", state.width, state.height);
    for i in 0..(state.height*state.width) {
        state.cells[i as usize] = random::<bool>() as u8;
    }

    let raw = Box::into_raw(Box::new(state));
    println!("return ptr {:?}", raw);
    return raw;
    // return Box::into_raw(Box::new(state)) as *mut libc::c_void;
}


#[no_mangle]
pub extern "C" fn next_state(state_ptr: *mut libc::c_void) -> *mut libc::c_char {
    println!("next_state ptr {:?}", state_ptr);
    // let state: &mut BoardState = unsafe { &mut *(state_ptr as *mut BoardState) };
    // let mut state = unsafe { Box::from_raw(state_ptr as *mut BoardState) };
    let mut state = unsafe { Box::from_raw(state_ptr as *mut BoardState) };

    let mut buff = String::from("");
    println!("{:?}", (state.width, state.height));
    state.gen_next();

    for (i, cell) in state.cells.iter().enumerate() {

        if *cell == LIVE {
            buff.push_str(LIVE_CHAR);
        } else {
            buff.push_str(DEAD_CHAR);
        }

        if  (i as i32  + 1) % state.width == 0 {
            buff.push_str("\n");
        }
    }

    let raw = Box::into_raw(Box::new(state));
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
