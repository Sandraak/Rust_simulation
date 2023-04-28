use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_mod_picking::{DebugCursorPickingPlugin, PickingCameraBundle, PickingPlugin};

use crate::board;
use crate::camera;
use crate::pieces;

pub fn create_app(screen_width: f32, screen_height: f32) -> App {
    let resolution = WindowResolution::new(screen_width, screen_height);
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: resolution.clone(),
            title: "Chess".to_owned(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(camera::PrimaryWindowResolution { resolution })
    .add_startup_system(setup)
    .add_startup_system(board::create_board)
    .add_startup_system(pieces::create_pieces)
    .add_system(camera::pan_orbit_camera)
    .add_plugin(PickingPlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(DebugCursorPickingPlugin);

    app
}

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        })
        .insert(PickingCameraBundle::default())
        .insert(camera::PanOrbitCamera::default());
    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}
