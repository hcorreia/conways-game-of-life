mod life;

use life::{Life, Shape, LIVE, LIVE_CHAR, DEAD_CHAR};
use libc;
use std::ffi::CString;
// use std::ffi::c_void;


#[no_mangle] // *mut libc::c_void
pub extern "C" fn init_state_empty(width: i32, height: i32, n_workers: i32) -> *mut Life {
    assert!(width > 0);
    assert!(height > 0);
    assert!(n_workers > 0);

    println!("{:?}, {:?}", width, height);
    let game = Life::new(width, height, Shape::Empty, n_workers as usize);
    println!("{:?}, {:?}", game.state.width, game.state.height);
    // for i in 0..(game.height*game.width) {
    //     game.cells[i as usize] = random::<bool>() as u8;
    // }

    let raw = Box::into_raw(Box::new(game));
    println!("return ptr {:?}", raw);
    return raw;
    // return Box::into_raw(Box::new(game)) as *mut libc::c_void;
}


#[no_mangle] // *mut libc::c_void
pub extern "C" fn init_state_random(width: i32, height: i32, n_workers: i32) -> *mut Life {
    assert!(width > 0);
    assert!(height > 0);
    assert!(n_workers > 0);

    println!("{:?}, {:?}", width, height);
    let game = Life::new(width, height, Shape::Random, n_workers as usize);
    println!("{:?}, {:?}", game.state.width, game.state.height);
    // for i in 0..(game.height*game.width) {
    //     game.cells[i as usize] = random::<bool>() as u8;
    // }

    let raw = Box::into_raw(Box::new(game));
    println!("return ptr {:?}", raw);
    return raw;
    // return Box::into_raw(Box::new(game)) as *mut libc::c_void;
}


#[no_mangle] // *mut libc::c_void
pub extern "C" fn init_state_glider(width: i32, height: i32, n_workers: i32) -> *mut Life {
    assert!(width > 0);
    assert!(height > 0);
    assert!(n_workers > 0);

    println!("{:?}, {:?}", width, height);
    let game = Life::new(width, height, Shape::Glider, n_workers as usize);
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
