mod lib;


fn main() {
    lib::life(
        // width, height
        // 80, 38,
        // 5, 10,
        2_000, 2_000,
        // max iter
        10_000,
        // sleep ms
        60,
        // debug, show time per tick insted of board
        true);

    // let s = lib::init_state_random_2(10, 5);

    // println!("main ptr {:?}", s);

    // let txt = lib::next_state(s);

    // println!("{}", txt);
    // println!("");
    // println!("{}", lib::next_state(s));
    // println!("");
    // println!("{}", lib::next_state(s));
    // println!("");
    // println!("{}", lib::next_state(s));
    // println!("");
    // println!("{}", lib::next_state(s));
}
