mod chess;
mod controller;
mod pathfinding;
mod simulation;
use crate::simulation::app;

fn main() {
    // let boardstate = chess::BoardState::default();
    // let path = calculate_path(TEST_MOVE, &boardstate);
    // print!("{:?}", path);
    let mut app = app::create_app(1600.0, 1600.0);
    app.run();
}
