mod app;
mod board;
mod camera;
mod pieces;

fn main() {
    let mut app = app::create_app(1600.0, 1600.0);
    app.run();
}
