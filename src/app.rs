use bevy::{prelude::*, window::WindowResolution};
use bevy_mod_picking::{DebugCursorPickingPlugin, DefaultPickingPlugins};
use bevy_rapier3d::{
    prelude::{RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use crate::{
    board::*,
    camera::{self, CameraPlugin},
    chess::*,
    frame::*,
    pieces::*,
};

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
            gravity: Vec3::new(0.0, -9.81, 0.0),
            ..default()
        })
        .insert_resource(camera::PrimaryWindowResolution { resolution })
        .add_system(camera::pan_orbit_camera)
        .add_plugin(RapierPhysicsPlugin::<()>::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(RapierDebugRenderPlugin {
            always_on_top: true,
            ..Default::default()
        })
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        .add_plugin(FramePlugin)
        .add_plugin(CameraPlugin);
    // .add_startup_system(setup);

    #[cfg(debug_assertions)]
    app.add_plugin(DebugCursorPickingPlugin);

    app
}
