use bevy::prelude::*;
use bevy::prelude::{shape::Box, *};
use bevy_rapier3d::prelude::RigidBody;

use crate::board::*;

const BAR_Y: f32 = 0.0;
const BAR_Z: f32 = 0.0;

const BAR_OFFSET: Vec3 = Vec3::new(
    0.5 * BOARD_WIDTH - 2.5,
    -0.5 * BOARD_HEIGHT,
    0.5 * BOARD_LENGTH - 1.5,
);

const MAGNET_HEIGHT: f32 = 0.25;
const MAGNET_RADIUS: f32 = 0.45;
const MAGNET_Y: f32 = -BOARD_HEIGHT - 0.5 * MAGNET_HEIGHT;
const MAGNET_OFFSET: Vec3 = Vec3::new(0.0, MAGNET_Y, 0.0);
pub struct FramePlugin;

impl Plugin for FramePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameColors>()
            .add_startup_system(create_frame)
            .add_startup_system(create_moving_bar)
            .add_startup_system(create_carrier)
            .add_startup_system(create_magnet)
            .add_system(move_magnet)
            .add_system(move_carrier)
            .add_system(move_bar);
    }
}
#[derive(Resource)]
struct FrameColors {
    frame: Handle<StandardMaterial>,
    bar: Handle<StandardMaterial>,
    carrier: Handle<StandardMaterial>,
    magnet: Handle<StandardMaterial>,
}

impl FromWorld for FrameColors {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        // let frame = materials.add(Color::rgb(0.2, 0.2, 1.0).into());
        let frame = materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 1.0),
            metallic: 1.0,
            ..default()
        });
        let bar = materials.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.5),
            metallic: 1.0,
            ..default()
        });
        let carrier = materials.add(Color::rgb(0.5, 0.3, 0.0).into());

        let magnet = materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 1.0),
            metallic: 1.0,
            ..default()
        });

        FrameColors {
            frame,
            bar,
            carrier,
            magnet,
        }
    }
}

#[derive(Component, Copy, Clone, Debug)]
struct Magnet {
    target_pos: Vec2,
}

#[derive(Component, Copy, Clone, Debug)]

pub struct Bar {
    pub target_pos: f32,
}
#[derive(Component, Copy, Clone, Debug)]
pub struct Carrier {
    target_pos: Vec2,
}

//Creates the magnet, which is a kinematic position based rigid body.
fn create_magnet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<FrameColors>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cylinder {
        height: MAGNET_HEIGHT,
        radius: MAGNET_RADIUS,
        ..default()
    }));
    commands
        .spawn(PbrBundle {
            mesh,
            material: colors.magnet.clone(),
            transform: Transform::from_translation(MAGNET_OFFSET),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Magnet {
            target_pos: Vec2 { x: 0.0, y: 0.0 },
        });
}

fn create_frame(commands: Commands, meshes: ResMut<Assets<Mesh>>, colors: Res<FrameColors>) {
    let frame_shapes = vec![
        Box {
            min_x: -2.5,
            max_x: -2.0,
            min_y: -1.75,
            max_y: -1.25,
            min_z: -3.5,
            max_z: 10.5,
        },
        Box {
            min_x: 9.0,
            max_x: 9.5,
            min_y: -1.75,
            max_y: -1.25,
            min_z: -3.5,
            max_z: 10.5,
        },
        Box {
            min_x: -2.5,
            max_x: 9.5,
            min_y: -1.75,
            max_y: -1.25,
            min_z: -3.5,
            max_z: -3.0,
        },
        Box {
            min_x: -2.5,
            max_x: 9.5,
            min_y: -1.75,
            max_y: -1.25,
            min_z: 10.0,
            max_z: 10.5,
        },
        Box {
            min_x: -2.5,
            max_x: -2.0,
            min_y: -1.25,
            max_y: -0.25,
            min_z: 9.0,
            max_z: 9.5,
        },
        Box {
            min_x: -2.5,
            max_x: -2.0,
            min_y: -1.25,
            max_y: -0.25,
            min_z: -2.5,
            max_z: -2.0,
        },
        Box {
            min_x: 9.5,
            max_x: 9.0,
            min_y: -1.25,
            max_y: -0.25,
            min_z: -2.5,
            max_z: -2.0,
        },
        Box {
            min_x: 9.5,
            max_x: 9.0,
            min_y: -1.25,
            max_y: -0.25,
            min_z: 9.5,
            max_z: 9.0,
        },
    ];
    create_part_of_frame(commands, meshes, colors, frame_shapes);
}

fn create_part_of_frame(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<FrameColors>,
    shapes: Vec<Box>,
) {
    for shape in shapes {
        let mesh = meshes.add(Mesh::from(shape)).clone();
        commands
            .spawn(PbrBundle {
                mesh,
                material: colors.frame.clone(),
                ..default()
            })
            .insert(RigidBody::Fixed);
    }
}

fn create_moving_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<FrameColors>,
) {
    let mesh = meshes.add(Mesh::from(shape::Box {
        min_x: 5.75,
        max_x: 6.25,
        min_y: -1.25,
        max_y: -1.0,
        min_z: -3.5,
        max_z: 10.5,
    }));
    commands
        .spawn(PbrBundle {
            mesh,
            material: colors.bar.clone(),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Bar { target_pos: -5.0 });
}

fn create_carrier(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<FrameColors>,
) {
    let mesh = meshes.add(Mesh::from(shape::Box {
        min_x: 6.0,
        max_x: 7.0,
        min_y: -1.0,
        max_y: -0.5,
        min_z: -3.5,
        max_z: -2.5,
    }));
    commands
        .spawn(PbrBundle {
            mesh,
            material: colors.carrier.clone(),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Carrier {
            target_pos: Vec2 { x: -5.0, y: 3.5 },
        });
}

fn move_magnet(
    time: Res<Time>,
    mut magnet_query: Query<(&mut Transform, &Magnet)>,
    mut bar_query: Query<(&mut Transform, With<Bar>)>,
    mut carrier_query: Query<(&mut Transform, &Carrier)>,
) {


    let (mut bar_transform, _) = bar_query.get_single_mut().unwrap();
    let (mut carrier_transform, _) = carrier_query.get_single_mut().unwrap();

    let (mut magnet_transform, _) = magnet_query.get_single_mut().unwrap();

    bar_transform.translation.x = magnet_transform.translation.x;
}

fn move_bar(time: Res<Time>, mut query: Query<(&mut Transform, &Bar)>) {
    for (mut transform, bar) in query.iter_mut() {
        let direction = Vec3::new(bar.target_pos, BAR_Y, BAR_Z) - transform.translation;
        if direction.length() > 0.05 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}

fn move_carrier(time: Res<Time>, mut query: Query<(&mut Transform, &Carrier)>) {
    for (mut transform, carrier) in query.iter_mut() {
        let direction =
            Vec3::new(carrier.target_pos.x, BAR_Y, carrier.target_pos.y) - transform.translation;
        if direction.length() > 0.05 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}
