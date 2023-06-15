// use pathfinding::{astar::{Node, self, a_star, START_NODE, END_NODE}};
use pathfinding::astar::*;
mod app;
mod board;
mod camera;
mod chess;
mod frame;
mod pathfinding;
mod pieces;

fn main() {
    let boardstate = chess::BoardState::default();
    let path = a_star(START_POS, END_POS, boardstate);
    print!("{:?}", path);
    // let mut app = app::create_app(1600.0, 1600.0);
    // app.run();
}
