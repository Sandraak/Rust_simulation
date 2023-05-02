use bevy::{prelude::*, window::WindowResolution};
use bevy_mod_picking::{DebugCursorPickingPlugin, DefaultPickingPlugins, PickingCameraBundle};

use crate::{board::*, camera, pieces::*, chess::*};

pub fn create_app(screen_width: f32, screen_height: f32) -> App {
    let resolution = WindowResolution::new(screen_width, screen_height);
    println!("starting app");
    let mut app = App::new();
    app.insert_resource(BoardState::default())
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: resolution.clone(),
            title: "Chess".to_owned(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(camera::PrimaryWindowResolution { resolution })
    .add_system(camera::pan_orbit_camera)
    .add_plugins(DefaultPickingPlugins)
    .add_plugin(BoardPlugin)
    .add_plugin(PiecesPlugin)
    .add_startup_system(setup);

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
