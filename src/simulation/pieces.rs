use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, Collider, ColliderMassProperties, Damping, ExternalForce, Friction,
    MassProperties, Restitution, RigidBody,
};

use crate::chess::{chess::Piece, BoardState};
use crate::controller::controller::MagnetStatus;
use crate::simulation::magnet::*;

const SPAWN_HEIGHT: f32 = 0.0;
const PIECES_HEIGHT: f32 = 1.75;
const PIECES_RADIUS: f32 = 0.20;
const PIECES_OFFSET: Vec3 = Vec3::new(0.0, SPAWN_HEIGHT + 0.66 * PIECES_HEIGHT, 0.0);
const PIECES_TRANSFORM: Vec3 = Vec3::new(0.2, 0.2, 0.2);
const PIECES_WEIGHT_CENTER: Vec3 = Vec3::new(0.0, 0.2 * PIECES_HEIGHT, 0.0);

const PIECES_MASS: f32 = 0.2;
const PIECES_FRICTION: f32 = 1.0;
const PIECES_RESTITUTION: f32 = 0.0;

/// plugin containing information about creating and moving pieces. This is used by the bevy app.
pub struct PiecesPlugin;


impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_pieces)
            .add_system(move_pieces);
    }
}

/// System that constantly checks the distance between a piece and the magnet.
/// It then executes a force on the piece towards the magnet,
/// The strength is based on the distance between the magnet and the piece. A smaller distance
/// results in a stronger force. The force is only executed when the magnet is on.
fn move_pieces(
    mut ext_forces: Query<(&mut ExternalForce, &mut Transform, With<PieceComponent>)>,
    magnet_query: Query<(&Transform, &Magnet, Without<PieceComponent>)>,
    magnet_status: Res<MagnetStatus>,
) {
    let (magnet_transform, _, _) = magnet_query.get_single().unwrap();
    if magnet_status.on {
        for (mut piece_force, piece_transform, _) in ext_forces.iter_mut() {
            let delta = magnet_transform.translation - piece_transform.translation;
            let direction = delta.normalize();
            let distance = delta.length();
            let force = direction * (MAGNET_STRENGTH / (4.0 * PI * distance.powf(2.0)));
            piece_force.force = force;
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PieceComponent {
    pub piece: Piece,
    pub target_x: usize,
    pub target_y: usize,
}
///Adds a collider to the parent, which in this case would be the piece.
///The collider contains information about the shape, restitution, mass, friction and position in relation the piece.
fn add_collider(parent: &mut ChildBuilder) {
    parent
        .spawn(Collider::cylinder(0.5 * PIECES_HEIGHT, PIECES_RADIUS))
        .insert(Restitution::coefficient(PIECES_RESTITUTION))
        .insert(ColliderMassProperties::MassProperties(MassProperties {
            local_center_of_mass: PIECES_WEIGHT_CENTER,
            mass: PIECES_MASS,
            ..default()
        }))
        .insert(Friction {
            coefficient: PIECES_FRICTION,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(Damping {
            linear_damping: 10.0,
            angular_damping: 10.0,
        })
        .insert(Transform::from_translation(PIECES_OFFSET));
}

/// Sets the transform such that the pieces face towards the center of the board.
fn set_piece_body_transform(piece: Piece) -> Transform {
    let piece_body_transform: Transform;
    match piece.kind {
        crate::chess::chess::Kind::Pawn => {
            piece_body_transform = Transform::from_translation(Vec3::new(-0.2, SPAWN_HEIGHT, 2.6))
        }
        crate::chess::chess::Kind::Rook => {
            piece_body_transform = Transform::from_translation(Vec3::new(-0.1, SPAWN_HEIGHT, 1.8))
        }
        crate::chess::chess::Kind::Knight => {
            piece_body_transform = Transform::from_translation(Vec3::new(-0.2, SPAWN_HEIGHT, 0.9))
        }
        crate::chess::chess::Kind::Bishop => {
            piece_body_transform = Transform::from_translation(Vec3::new(-0.1, SPAWN_HEIGHT, 0.0))
        }
        crate::chess::chess::Kind::Queen => {
            piece_body_transform = Transform::from_translation(Vec3::new(-0.2, SPAWN_HEIGHT, -0.95))
        }
        crate::chess::chess::Kind::King => {
            piece_body_transform = Transform::from_translation(Vec3::new(-0.2, SPAWN_HEIGHT, -1.9))
        }
    }
    piece_body_transform.with_scale(PIECES_TRANSFORM)
}

/// Sets the position as a transform.
fn set_position(position: (usize, usize)) -> Transform {
    Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32))
}

/// Creates a PieceComponent from a Piece and a position.
fn set_piece(piece: Piece, position: (usize, usize)) -> PieceComponent {
    PieceComponent {
        piece,
        target_x: position.0,
        target_y: position.1,
    }
}

fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: (usize, usize),
) {
    commands
        .spawn(PbrBundle {
            transform: set_position(position),
            ..Default::default()
        })
        .insert(set_piece(piece, position))
        .insert(ExternalForce::default())
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material: material.clone(),
                transform: set_piece_body_transform(piece),
                ..Default::default()
            });
            parent.spawn(PbrBundle {
                mesh: mesh_cross,
                material,
                transform: set_piece_body_transform(piece),
                ..Default::default()
            });
        })
        .insert(RigidBody::Dynamic)
        .with_children(add_collider);
}

pub fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    position: (usize, usize),
) {
    commands
        .spawn(PbrBundle {
            transform: set_position(position),
            ..Default::default()
        })
        .insert(set_piece(piece, position))
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce::default())
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: mesh_1,
                material: material.clone(),
                transform: set_piece_body_transform(piece),
                ..Default::default()
            });
            parent.spawn(PbrBundle {
                mesh: mesh_2,
                material,
                transform: set_piece_body_transform(piece),
                ..Default::default()
            });
        })
        .insert(RigidBody::Dynamic)
        .with_children(add_collider);
}

pub fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    mesh: Handle<Mesh>,
    position: (usize, usize),
) {
    commands
        .spawn(PbrBundle {
            transform: set_position(position),
            ..Default::default()
        })
        .insert(set_piece(piece, position))
        .insert(ExternalForce::default())
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: set_piece_body_transform(piece),
                ..Default::default()
            });
        })
        .insert(RigidBody::Dynamic)
        .with_children(add_collider);
}

pub fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    mesh: Handle<Mesh>,
    position: (usize, usize),
) {
    commands
        .spawn(PbrBundle {
            transform: set_position(position),
            ..Default::default()
        })
        .insert(set_piece(piece, position))
        .insert(ExternalForce::default())
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: set_piece_body_transform(piece),
                ..Default::default()
            });
        })
        .insert(RigidBody::Dynamic)
        .with_children(add_collider);
}

pub fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    mesh: Handle<Mesh>,
    position: (usize, usize),
) {
    commands
        .spawn(PbrBundle {
            transform: set_position(position),
            ..Default::default()
        })
        .insert(set_piece(piece, position))
        .insert(ExternalForce::default())
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: set_piece_body_transform(piece),
                ..Default::default()
            });
        })
        .insert(RigidBody::Dynamic)
        .with_children(add_collider);
}

pub fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    mesh: Handle<Mesh>,
    position: (usize, usize),
) {
    commands
        .spawn(PbrBundle {
            transform: set_position(position),
            ..Default::default()
        })
        .insert(set_piece(piece, position))
        .insert(ExternalForce::default())
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: set_piece_body_transform(piece),
                ..Default::default()
            });
        })
        .insert(RigidBody::Dynamic)
        .insert(Friction {
            coefficient: PIECES_FRICTION,
            combine_rule: CoefficientCombineRule::Max,
        })
        .with_children(add_collider);
}

/// Spawns all the pieces on the location they have in the [`BoardState`].
fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    state: Res<BoardState>,
) {
    let king_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh7/Primitive0");

    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    for (position, piece) in state
        .chess
        .board
        .iter()
        .enumerate()
        .flat_map(|(row, pieces)| {
            pieces
                .iter()
                .enumerate()
                .map(move |(col, piece)| ((row, col), piece))
        })
    {
        if let Some(piece) = piece {
            let material = match piece.color {
                crate::chess::chess::Color::Black => black_material.clone(),
                crate::chess::chess::Color::White => white_material.clone(),
            };
            match piece.kind {
                crate::chess::chess::Kind::Pawn => spawn_pawn(
                    &mut commands,
                    material,
                    *piece,
                    pawn_handle.clone(),
                    position,
                ),
                crate::chess::chess::Kind::Rook => spawn_rook(
                    &mut commands,
                    material,
                    *piece,
                    rook_handle.clone(),
                    position,
                ),
                crate::chess::chess::Kind::Knight => spawn_knight(
                    &mut commands,
                    material,
                    *piece,
                    knight_1_handle.clone(),
                    knight_2_handle.clone(),
                    position,
                ),
                crate::chess::chess::Kind::Queen => spawn_queen(
                    &mut commands,
                    material,
                    *piece,
                    queen_handle.clone(),
                    position,
                ),
                crate::chess::chess::Kind::King => spawn_king(
                    &mut commands,
                    material,
                    *piece,
                    king_handle.clone(),
                    king_cross_handle.clone(),
                    position,
                ),
                crate::chess::chess::Kind::Bishop => spawn_bishop(
                    &mut commands,
                    material,
                    *piece,
                    bishop_handle.clone(),
                    position,
                ),
            };
        }
    }
}
