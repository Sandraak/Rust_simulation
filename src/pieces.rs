use bevy::prelude::*;

use crate::chess::{chess::Piece, BoardState};

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_pieces);
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
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece.color,
            kind: piece.kind,
        })
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.with_scale(Vec3::new(0.2, 0.2, 0.2))
                },
                ..Default::default()
            });
            parent.spawn(PbrBundle {
                mesh: mesh_cross,
                material,
                transform: {
                    let transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.with_scale(Vec3::new(0.2, 0.2, 0.2))
                },
                ..Default::default()
            });
        });
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
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece.color,
            kind: piece.kind,
        })
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: mesh_1,
                material: material.clone(),
                transform: {
                    let transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                    transform.with_scale(Vec3::new(0.2, 0.2, 0.2))
                },
                ..Default::default()
            });
            parent.spawn(PbrBundle {
                mesh: mesh_2,
                material,
                transform: {
                    let transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                    transform.with_scale(Vec3::new(0.2, 0.2, 0.2))
                },
                ..Default::default()
            });
        });
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
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece.color,
            kind: piece.kind,
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                    transform.with_scale(Vec3::new(0.2, 0.2, 0.2))
                },
                ..Default::default()
            });
        });
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
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece.color,
            kind: piece.kind,
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.));
                    transform.with_scale(Vec3::new(0.2, 0.2, 0.2))
                },
                ..Default::default()
            });
        });
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
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece.color,
            kind: piece.kind,
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
                    transform.with_scale(Vec3::new(0.2, 0.2, 0.2))
                },
                ..Default::default()
            });
        });
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
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece.color,
            kind: piece.kind,
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                    transform.with_scale(Vec3::new(0.2, 0.2, 0.2))
                },
                ..Default::default()
            });
        });
}

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    state: ResMut<BoardState>,
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

    for (position, piece) in state.chess.board.iter().enumerate().flat_map(|(x, row)| {
        row.iter()
            .enumerate()
            .map(move |(y, piece)| ((x, y), piece))
    }) {
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

// fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
//     for (mut transform, piece) in query.iter_mut() {
//         // Get the direction to move in
//         let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;

//         // Only move if the piece isn't already there (distance is big)
//         if direction.length() > 0.1 {
//             transform.translation += direction.normalize() * time.delta_seconds();
//         }
//     }
// }
