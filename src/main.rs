mod chess;
mod controller;
mod pathfinding;
mod simulation;
use crate::simulation::app;

fn main() {
    let mut app = app::create_app(1600.0, 1600.0);
    app.run();
}
