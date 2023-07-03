use crate::{chess::pos::Pos, simulation::frame::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::board::BOARD_HEIGHT;

const MAGNET_HEIGHT: f32 = 0.5;
const MAGNET_RADIUS: f32 = 0.25;
const MAGNET_Y: f32 = -BOARD_HEIGHT - 0.5 * MAGNET_HEIGHT;
const MAGNET_OFFSET: Vec3 = Vec3::new(-1.25, MAGNET_Y, -1.25);

pub const MAGNET_STRENGTH: f32 = 25.0;
pub struct MagnetPlugin;

impl Plugin for MagnetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameColors>()
            .add_startup_system(create_magnet)
            .add_system(move_magnet);
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Magnet {
    target_pos: Vec2,
    pub on: bool,
    positions_reached: usize,
}

#[derive(Resource)]
struct NextPosition {
    destination: Pos,
}

///System that constantly checks the distance between the desired and true position of magnet.
/// It moves the magnet towards the desired position as long as this distance is larger than 0.01.
fn move_magnet(
    time: Res<Time>,
    mut magnet_query: Query<(&mut Transform, &mut Magnet, Without<Bar>, Without<Carrier>)>,
    // positions: Vec<Pos>,
) {
    let (mut magnet_transform, magnet, _, _) = magnet_query.get_single_mut().unwrap();
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
