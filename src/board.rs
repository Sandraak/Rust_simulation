use bevy::prelude::*;
use bevy_mod_picking::{Hover, PickableBundle, Selection};
use bevy_rapier3d::{
    prelude::{Collider, Restitution, RigidBody},
    rapier::prelude::{
        ColliderBuilder, ColliderSet, RigidBodyBuilder, RigidBodySet, RigidBodyType,
    },
};

use crate::pieces::PieceComponent;

pub struct BoardPlugin;

const BOARD_LENGTH: f32 = 10.0;
const BOARD_WIDTH: f32 = 12.0;
const BOARD_HEIGHT: f32 = 0.25;
const SMALL_FLOAT: f32 = 0.01;

const BOARD_OFFSET: Vec3 = Vec3::new(
    0.5 * BOARD_WIDTH - 2.5,
    -0.5 * BOARD_HEIGHT,
    0.5 * BOARD_LENGTH - 1.5,
);

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SquareColors>()
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create_board)
            .add_startup_system(create_border)
            .add_system(color_squares)
            .add_system(perform_move);
    }
}

#[derive(Resource)]
struct SquareColors {
    white: Handle<StandardMaterial>,
    black: Handle<StandardMaterial>,
    white_hovered: Handle<StandardMaterial>,
    black_hovered: Handle<StandardMaterial>,
    white_selected: Handle<StandardMaterial>,
    black_selected: Handle<StandardMaterial>,
    border: Handle<StandardMaterial>,
}

impl FromWorld for SquareColors {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let white = materials.add(Color::rgb(0.7, 0.6, 0.6).into());
        let black = materials.add(Color::rgb(0.3, 0.2, 0.2).into());
        let white_hovered = materials.add(Color::rgb(0.8, 0.7, 0.7).into());
        let black_hovered = materials.add(Color::rgb(0.4, 0.3, 0.3).into());
        let white_selected = materials.add(Color::rgb(0.8, 0.7, 1.0).into());
        let black_selected = materials.add(Color::rgb(0.4, 0.3, 0.6).into());
        let border = materials.add(Color::rgb(0., 0.2, 0.2).into());

        SquareColors {
            white,
            black,
            white_hovered,
            black_hovered,
            white_selected,
            black_selected,
            border,
        }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Default, Resource)]
struct SelectedSquare {
    selected: Option<Square>,
}

#[derive(Default, Resource, Debug)]
struct SelectedPiece {
    selected: Option<Entity>,
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<SquareColors>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.,
        ..default()
    }));

    // Spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    // Change material according to position to get alternating pattern
                    // hier algemenere functie is_white.
                    material: if (i + j + 1) % 2 == 0 {
                        colors.white.clone()
                    } else {
                        colors.black.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .insert(PickableBundle::default())
                .insert(Square { x: i, y: j });
        }
    }
}

fn color_squares(
    mut query: Query<(&Square, &mut Handle<StandardMaterial>, &Hover, &Selection)>,
    colors: Res<SquareColors>,
) {
    for (square, mut handle, hover, selection) in query.iter_mut() {
        if selection.selected() {
            *handle = if square.is_white() {
                colors.white_selected.clone()
            } else {
                colors.black_selected.clone()
            }
        } else if hover.hovered() {
            *handle = if square.is_white() {
                colors.white_hovered.clone()
            } else {
                colors.black_hovered.clone()
            }
        }
    }
}

fn create_border(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<SquareColors>,
) {
    let mesh = meshes.add(Mesh::from(shape::Box {
        min_x: -1.5,
        max_x: 8.5,
        min_y: -0.25,
        max_y: -SMALL_FLOAT,
        min_z: -2.5,
        max_z: 9.5,
    }));
    commands
        .spawn(PbrBundle {
            mesh,
            material: colors.border.clone(),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(
                    0.5 * BOARD_LENGTH,
                    0.5 * BOARD_HEIGHT,
                    0.5 * BOARD_WIDTH,
                ))
                .insert(Transform::from_translation(BOARD_OFFSET))
                .insert(Restitution::coefficient(0.0));
        });
}

fn perform_move(
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut square_query: Query<(&Square, &Interaction, Entity)>,
    mut pieces_query: Query<(&mut PieceComponent, Entity)>,
) {
    // Only run if the left button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    //selects the piece that will be moved
    for (square, interaction, _entity) in square_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            let optional_piece = pieces_query
                .into_iter()
                .find(|piece| piece.0.x as u8 == square.x && piece.0.y as u8 == square.y);
            if optional_piece.is_some() {
                // Add the identifier of the piece entity to selected_piece. This identifier is later used to query the location of the selected piece.
                selected_piece.selected = Some(optional_piece.unwrap().1);
                info!("selected piece: {:?}", optional_piece);
                //return so that the selected square won't be the same as the square the selected piece is on.
                return;
            }
        }
    }
    // When a piece is selected, selects a square to where the selected piece will move.
    if selected_piece.selected.is_some() {
        for (square, interaction, _entity) in square_query.iter_mut() {
            if let Interaction::Clicked = interaction {
                selected_square.selected = Some(*square);
                info!("selected square: {:?}", selected_square.selected);
            }
        }
    }

    if selected_piece.selected.is_some() && selected_square.selected.is_some() {
        // Get the PieceComponent of the piece with the identifier that was specified earlier.
        let (mut selected_piece_com, _) = pieces_query
            .get_mut(selected_piece.selected.unwrap())
            .unwrap();
        selected_piece_com.x = selected_square.selected.unwrap().x as usize;
        selected_piece_com.y = selected_square.selected.unwrap().y as usize;

        selected_piece.selected = None;
        selected_square.selected = None;
    }
}
