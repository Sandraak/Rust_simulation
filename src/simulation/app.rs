use bevy::{prelude::*, window::WindowResolution};
use bevy_mod_picking::{DebugCursorPickingPlugin, DefaultPickingPlugins};
use bevy_rapier3d::{
    prelude::{RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use crate::{
    chess::{computer::ChessComputerPlugin, *},
    controller::{controller::ControllerPlugin, ui::UserInterfacePlugin},
    pathfinding::astar::PathfindingPlugin,
    simulation::board::*,
    simulation::camera::{self, CameraPlugin},
    simulation::frame::*,
    simulation::pieces::*,
};

use super::magnet::MagnetPlugin;

pub fn create_app(screen_width: f32, screen_height: f32) -> App {
    let resolution = WindowResolution::new(screen_width, screen_height);
    let mut app = App::new();
    app.insert_resource(BoardState::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: resolution.clone(),
                title: "Automatic chessboard simulation".to_owned(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(RapierConfiguration {
            gravity: Vec3::new(0.0, -98.1, 0.0),
            ..default()
        })
        .insert_resource(camera::PrimaryWindowResolution { resolution })
        .add_plugin(RapierPhysicsPlugin::<()>::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(RapierDebugRenderPlugin {
            always_on_top: true,
            ..Default::default()
        })
        .add_plugin(ControllerPlugin)
        .add_plugin(PathfindingPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(MagnetPlugin)
        .add_plugin(PiecesPlugin)
        .add_plugin(FramePlugin)
        .add_plugin(UserInterfacePlugin)
        .add_plugin(ChessComputerPlugin)
        .add_plugin(CameraPlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(DebugCursorPickingPlugin);

    app
}
