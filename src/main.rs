use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_mod_picking::{
    DebugCursorPickingPlugin, PickableBundle, PickingCameraBundle, PickingPlugin,
};

mod pieces;
mod camera;
use camera::*;

fn main() {
    let resolution = WindowResolution::new(1600.0, 1600.0);
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution:resolution.clone(),
            title: "Chess".to_owned(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(PrimaryWindowResolution{resolution
    })
    .add_startup_system(setup)
    .add_startup_system(create_board)
    .add_startup_system(pieces::create_pieces)
    .add_system(camera::pan_orbit_camera)
    .add_plugin(PickingPlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(DebugCursorPickingPlugin);

    app.run();
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
        .insert(PanOrbitCamera::default());
    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.,
        ..default()
    }));
    let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

    // Spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    // Change material according to position to get alternating pattern
                    material: if (i + j + 1) % 2 == 0 {
                        white_material.clone()
                    } else {
                        black_material.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .insert(PickableBundle::default());
        }
    }
}


