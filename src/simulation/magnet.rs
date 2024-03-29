use crate::{
    controller::controller::{Destination, MagnetEvent, MagnetStatus},
    simulation::frame::*,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::board::BOARD_HEIGHT;

const MAGNET_HEIGHT: f32 = 0.25;
const MAGNET_RADIUS: f32 = 0.25;
const MAGNET_Y: f32 = -BOARD_HEIGHT - 0.3 * MAGNET_HEIGHT;
const MAGNET_OFFSET: Vec3 = Vec3::new(-2.25, MAGNET_Y, -2.25);

pub const MAGNET_STRENGTH: f32 = 7.5;

/// Plugin containing information about the magnet, this is used by the bevy app.
pub struct MagnetPlugin;

impl Plugin for MagnetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameColors>()
            .add_startup_system(create_magnet)
            .add_system(move_magnet)
            .add_system(signaler);
    }
}

/// The position of the magnet.
#[derive(Component, Copy, Clone, Debug)]
pub struct Magnet {
    target_pos: Vec2,
}

/// System that checks whether the magnet has reached it's destination.
/// When this is the case, this function sends a [`MagnetEvent`] which triggers
/// update_current_pos in controller.rs
fn signaler(
    magnet_query: Query<(&Transform, &Magnet, Without<Bar>, Without<Carrier>)>,
    mut magnet_update: EventWriter<MagnetEvent>,
    mut magnet_status: ResMut<MagnetStatus>,
    destination: Res<Destination>,
) {
    let (magnet_transform, _magnet, _, _) = magnet_query.get_single().unwrap();
    let magnet_direction = Vec3::new(
        destination.goal.y() as f32,
        MAGNET_Y,
        destination.goal.x() as f32,
    ) - magnet_transform.translation;

    if magnet_direction.length() <= 0.01 && !magnet_status.simulation {
        magnet_status.simulation = true;
        magnet_update.send(MagnetEvent);
    }
}

/// System that constantly checks the distance between the desired and true position of magnet.
/// It moves the magnet towards the desired position as long as this distance is larger than 0.01.
fn move_magnet(
    time: Res<Time>,
    mut magnet_query: Query<(&mut Transform, &mut Magnet, Without<Bar>, Without<Carrier>)>,
    destination: Res<Destination>,
) {
    let (mut magnet_transform, mut magnet, _, _) = magnet_query.get_single_mut().unwrap();
    let magnet_direction = Vec3::new(magnet.target_pos.y, MAGNET_Y, magnet.target_pos.x)
        - magnet_transform.translation;
    if magnet_direction.length() > 0.01 {
        magnet_transform.translation += magnet_direction.normalize() * time.delta_seconds();
    } else {
        // goal reached
        magnet.target_pos.x = destination.goal.x() as f32;
        magnet.target_pos.y = destination.goal.y() as f32;
    }
}

/// Creates the magnet, which is a kinematic positition based rigid body.
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
        });
}
