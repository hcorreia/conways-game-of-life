use std::thread;
use std::time;

const LIVE: i32 = 1;
const DEAD: i32 = 0;

const LIVE_CHAR: &str = "\u{2588}\u{2588}";  // \u2588 OR \u2588
const DEAD_CHAR: &str = "\u{2591}\u{2591}";


pub struct BoardState {
    width: u32,
    height: u32,
    cells: Vec<Vec<i32>>,
}


fn init_state_empty() -> Vec<Vec<i32>> {
    let width = 15;
    let height = 10;
    let state = vec![vec![0; width]; height];

    return state;
}

fn init_state_random() -> Vec<Vec<i32>> {
    let mut state = init_state_empty();

    state[2][0] = 1;
    state[2][1] = 1;
    state[2][2] = 1;
    state[1][2] = 1;
    state[0][1] = 1;

    return state;
}

// fn is_cell_alive(state: Vec<Vec<i32>>, x: usize, y: usize, width: usize, height: usize) -> bool {
//     // println!("{:?}", (x, y));
//     if x >= 0 && x < width && y >= 0 && y < height {
//         return state[y][x] == LIVE;
//     }
//     return false;
// }

fn get_index(state: Vec<Vec<i32>>, x: i32, y: i32, width: i32, height: i32) -> i32 {
    if x >= 0 && x < width && y >= 0 && y < height {
        return state[y as usize][x as usize];
    }
    return DEAD;
}
fn get_neighbors(state: Vec<Vec<i32>>, x: i32, y: i32, width: i32, height: i32) -> i32 {
    let mut neighbors = 0;

    for dy in [-1, 0, 1].iter().cloned() {
        for dx in [-1, 0, 1].iter().cloned() {
            // println!("deltas {:?}", (dx, dy));
            if dx == 0 && dy == 0 {
                continue;
            }
            // println!("TEST {:?}", (x+dx, y+dy));
            if get_index(state.clone(), x+dx, y+dy, width, height) == LIVE {
                neighbors += 1;
            }
        }
    }

    return neighbors;
}

fn gen_next(state: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let width = 15;
    let height = 10;
    let mut next_state = init_state_empty();
    let mut neighbors: i32;

    for y in 0..height {
        for x in 0..width {
            neighbors = get_neighbors(state.clone(), x, y, width, height);

            if state[y as usize][x as usize] == LIVE {
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

    return next_state;
}

fn draw(state: Vec<Vec<i32>>) {
    println!("");
    println!("");
    println!("");
    println!("");

    for row in state {
        let mut line = String::from("");

        for cel in row {
            if cel == LIVE {
                line.push_str(LIVE_CHAR);
            } else {
                line.push_str(DEAD_CHAR);
            }
        }
        println!("{}", line);
        line.clear();
    }
}

fn life(limit: i64, wait: u64) {
    let mut state = init_state_random();

    draw(state.clone());
    // draw(state.clone());
    for _ in 0..limit {
        thread::sleep(time::Duration::from_millis(wait));
        state = gen_next(state.clone());
        draw(state.clone());
    }
    // draw(state);
}

fn main() {
    life(100, 120);

    // let a: usize = 0;
    // let b: u64 = a-1;

    // for x in 0..10 {
    //     println!("{:?}", x-1);
    // }

    // println!("{:?}", (a, b));
}
