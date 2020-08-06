mod life;


fn main() {
    life::start_life(
        // width, height
        80, 38,
        // 5, 10,
        // 2_000, 2_000,
        // init shape
        // life::Shape::Empty,
        life::Shape::Random,
        // life::Shape::Glider,
        // Worker threads
        4,
        // max iter
        10_100,
        // sleep ms
        60,
        // debug, show time per tick insted of board
        false);
}
