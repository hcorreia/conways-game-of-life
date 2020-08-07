mod life;


fn main() {
    life::start_life(
        // width, height
        // 7, 3,
        80, 38,
        // 230, 100,
        // 5_000, 5_000,
        // init shape
        // life::Shape::Empty,
        life::Shape::Random,
        // life::Shape::Glider,
        // Worker threads
        1,
        // max iter
        10_000,
        // sleep ms
        120,
        // debug, show time per tick insted of board
        false);
}
