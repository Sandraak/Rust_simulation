use core::time;
use std::thread;

// use bevy::prelude::*;
use bevy::prelude::{shape::Box, *};
use bevy_rapier3d::prelude::RigidBody;

use crate::{board::*, chess::pos::Pos};

const BAR_Y: f32 = 0.0;
const BAR_Z: f32 = 0.0;
const BAR_OFFSET: Vec3 = Vec3::new(0.0, BAR_Y, BAR_Z);

const CARRIER_Y: f32 = 0.0;
const CARRIER_OFFSET: Vec3 = Vec3::new(0.0, CARRIER_Y, 0.0);

const MAGNET_HEIGHT: f32 = 0.5;
const MAGNET_RADIUS: f32 = 0.25;
const MAGNET_Y: f32 = -BOARD_HEIGHT - 0.5 * MAGNET_HEIGHT;
const MAGNET_OFFSET: Vec3 = Vec3::new(-1.25, MAGNET_Y, -1.25);
pub const MAGNET_STRENGTH: f32 = 50.0;
pub struct FramePlugin;

impl Plugin for FramePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameColors>()
            .add_startup_system(create_frame)
            .add_startup_system(create_moving_bar)
            .add_startup_system(create_carrier)
            .add_startup_system(create_magnet)
            .add_system(move_magnet);
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
            base_color: Color::rgb(0.9, 0.9, 0.9),
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
pub struct Magnet {
    target_pos: Vec2,
    pub on: bool,
    positions_reached: usize,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Bar {
    pub target_pos: f32,
}
#[derive(Component, Copy, Clone, Debug)]
pub struct Carrier {
    _target_pos: Vec2,
}

//Creates the magnet, which is a cylinder shaped, kinematic position based rigid body.
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
            target_pos: Vec2 {
                x: MAGNET_OFFSET[0],
                y: MAGNET_OFFSET[2],
            },
            on: false,
            positions_reached: 0,
        });
}

//Creates the moving bar that carries the carrier, which is a box shaped, kinematic position based rigid body.
fn create_moving_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<FrameColors>,
) {
    let mesh = meshes.add(Mesh::from(shape::Box {
        min_x: -1.5,
        max_x: -1.0,
        min_y: -1.25,
        max_y: -1.0,
        min_z: -3.5,
        max_z: 10.5,
    }));
    commands
        .spawn(PbrBundle {
            mesh,
            material: colors.bar.clone(),
            transform: Transform::from_translation(BAR_OFFSET),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Bar {
            target_pos: BAR_OFFSET[0],
        });
}

//Creates the carrier that carries the magnet, which is a box shaped, kinematic position based rigid body.
fn create_carrier(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<FrameColors>,
) {
    let mesh = meshes.add(Mesh::from(shape::Box {
        min_x: -1.75,
        max_x: -0.75,
        min_y: -1.0,
        max_y: -0.75,
        min_z: -1.75,
        max_z: -0.75,
    }));
    commands
        .spawn(PbrBundle {
            mesh,
            material: colors.carrier.clone(),
            transform: Transform::from_translation(CARRIER_OFFSET),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Carrier {
            _target_pos: Vec2 {
                x: CARRIER_OFFSET[0],
                y: CARRIER_OFFSET[2],
            },
        });
}
///System that constantly checks the distance between the desired and true position of magnet.
/// It moves the magnet towards the desired position as long as this distance is larger than 0.01.
fn move_magnet(
    time: Res<Time>,
    mut magnet_query: Query<(&mut Transform, &mut Magnet, Without<Bar>, Without<Carrier>)>,
    mut bar_query: Query<(&mut Transform, With<Bar>, Without<Magnet>, Without<Carrier>)>,
    mut carrier_query: Query<(&mut Transform, With<Carrier>, Without<Magnet>, Without<Bar>)>,
    // positions: Vec<Pos>,
) {
    let (mut bar_transform, _, _, _) = bar_query.get_single_mut().unwrap();
    let (mut carrier_transform, _, _, _) = carrier_query.get_single_mut().unwrap();
    let (mut magnet_transform, magnet, _, _) = magnet_query.get_single_mut().unwrap();

    bar_transform.translation.x = magnet_transform.translation.x + 1.25;
    carrier_transform.translation.x = magnet_transform.translation.x + 1.25;
    carrier_transform.translation.z = magnet_transform.translation.z + 1.25;

    // test data
    // Will need to receive path from pathfinding.
    let positions: Vec<Pos> = vec![
        Pos { x: 1, y: 1 },
        Pos { x: 2, y: 2 },
        Pos { x: 2, y: 3 },
        Pos { x: 2, y: 4 },
        Pos { x: 3, y: 4 },
        Pos { x: 4, y: 4 },
        Pos { x: 5, y: 4 },
    ];

    let magnet_direction = Vec3::new(magnet.target_pos.x, MAGNET_Y, magnet.target_pos.y)
        - magnet_transform.translation;
    // Move magnet towards goal
    if magnet_direction.length() > 0.01 {
        magnet_transform.translation += magnet_direction.normalize() * time.delta_seconds();
    } else {
        info!(
            "goal: {:?}, {:?} reached, magnet status: {:?}",
            magnet.target_pos.x, magnet.target_pos.y, magnet.on
        );
        // thread::sleep(time::Duration::from_millis(1000));
        if let Ok((_, mut magnet, _, _)) = magnet_query.get_single_mut() {
            if magnet.positions_reached < positions.len() {
                magnet.target_pos.x = positions[magnet.positions_reached].x as f32;
                magnet.target_pos.y = positions[magnet.positions_reached].y as f32;
                info!(
                    "Moving to {:?}, {:?}, magnet status: {:?}",
                    magnet.target_pos.x, magnet.target_pos.y, magnet.on
                );
                magnet.positions_reached += 1;
                magnet.on = true;
            } else {
                // End goal has been reached,
                magnet.positions_reached = 0;
                magnet.on = false;
                info!(
                    "END goal: {:?}, {:?} reached, magnet status: {:?},",
                    magnet.target_pos.x, magnet.target_pos.y, magnet.on
                );
                //Need to wait for new vector with positions. For now loop, so start again at 0 without turning on the magnet.
                magnet.target_pos.x = positions[magnet.positions_reached].x as f32;
                magnet.target_pos.y = positions[magnet.positions_reached].y as f32;
                // Now the function will loop again because magnet.positions_reached = 0;
            }
        }
    }
}
///The frame consists of 4 bars and 4 pillars on which the board rests, this function creates the frame.
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

/// Creates the subshapes out of which the whole frame exists.
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
