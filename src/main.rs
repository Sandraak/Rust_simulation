mod chess;
mod pathfinding;
mod simulation;
use crate::simulation::app;
use pathfinding::astar::*;

fn main() {
    // let boardstate = chess::BoardState::default();
    // let path = calculate_path(START_POS, END_POS, &boardstate);
    // print!("{:?}", path);
    let mut app = app::create_app(1600.0, 1600.0);
    app.run();
}
