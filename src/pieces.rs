use bevy::prelude::*;

use crate::chess::BoardState;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_pieces)
            .add_system(move_pieces);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Component, Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // Current position
    pub x: u8,
    pub y: u8,
}

fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: (u8, u8),
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
            color: piece_color,
            piece_type: PieceType::King,
            x: position.0,
            y: position.1,
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
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    position: Vec3,
) {
    commands
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
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
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
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
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
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
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
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
            color: piece_color,
            piece_type: PieceType::Rook,
            x: position.0,
            y: position.1,
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
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
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
    mut state: ResMut<BoardState>
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

    spawn_rook(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        rook_handle.clone(),
        (0, 0),
    );
    spawn_knight(
        &mut commands,
        white_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(0., 0., 1.),
    );
    spawn_bishop(
        &mut commands,
        white_material.clone(),
        bishop_handle.clone(),
        Vec3::new(0., 0., 2.),
    );
    spawn_queen(
        &mut commands,
        white_material.clone(),
        queen_handle.clone(),
        Vec3::new(0., 0., 3.),
    );
    spawn_king(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        king_handle.clone(),
        king_cross_handle.clone(),
        (0,4),
    );
    spawn_bishop(
        &mut commands,
        white_material.clone(),
        bishop_handle.clone(),
        Vec3::new(0., 0., 5.),
    );
    spawn_knight(
        &mut commands,
        white_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(0., 0., 6.),
    );
    spawn_rook(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        rook_handle.clone(),
        (0, 7),
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            white_material.clone(),
            pawn_handle.clone(),
            Vec3::new(1., 0., i as f32),
        );
    }

    spawn_rook(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        rook_handle.clone(),
        (7, 0),
    );
    spawn_knight(
        &mut commands,
        black_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(7., 0., 1.),
    );
    spawn_bishop(
        &mut commands,
        black_material.clone(),
        bishop_handle.clone(),
        Vec3::new(7., 0., 2.),
    );
    spawn_queen(
        &mut commands,
        black_material.clone(),
        queen_handle.clone(),
        Vec3::new(7., 0., 3.),
    );
    spawn_king(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        king_handle.clone(),
        king_cross_handle.clone(),
        (7,4),
    );
    spawn_bishop(
        &mut commands,
        black_material.clone(),
        bishop_handle.clone(),
        Vec3::new(7., 0., 5.),
    );
    spawn_knight(
        &mut commands,
        black_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(7., 0., 6.),
    );
    spawn_rook(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        rook_handle.clone(),
        (7, 7),
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            black_material.clone(),
            pawn_handle.clone(),
            Vec3::new(6., 0., i as f32),
        );
    }
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        // Get the direction to move in
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}


