use bevy::prelude::*;
use bevy_mod_picking::{Hover, PickableBundle, Selection};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction, RigidBody};

pub const SMALL_FLOAT: f32 = 0.01;
const BOARD_FRICTION: f32 = 0.8;

pub const BOARD_LENGTH: f32 = 12.0;
pub const BOARD_WIDTH: f32 = 14.0;
pub const BOARD_HEIGHT: f32 = 0.25;
const BOARD_OFFSET: Vec3 = Vec3::new(
    0.5 * BOARD_WIDTH - 3.5,
    -0.5 * BOARD_HEIGHT,
    0.5 * BOARD_LENGTH - 2.5,
);

//Plugin for creating the board and lighting up the squares
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardColors>()
            .add_startup_system(create_board)
            .add_startup_system(create_border)
            .add_system(color_squares);
    }
}

///Struct with all colors used for the board.
#[derive(Resource)]
struct BoardColors {
    white: Handle<StandardMaterial>,
    black: Handle<StandardMaterial>,
    white_hovered: Handle<StandardMaterial>,
    black_hovered: Handle<StandardMaterial>,
    white_selected: Handle<StandardMaterial>,
    black_selected: Handle<StandardMaterial>,
    border: Handle<StandardMaterial>,
}

impl FromWorld for BoardColors {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let white = materials.add(Color::rgba(1.0, 1.0, 1.0, 0.5).into());
        let black = materials.add(Color::rgba(0.0, 0.0, 0.0, 0.5).into());
        let white_hovered = materials.add(Color::rgba(0.8, 0.7, 0.7, 0.5).into());
        let black_hovered = materials.add(Color::rgba(0.4, 0.3, 0.3, 0.5).into());
        let white_selected = materials.add(Color::rgba(0.8, 0.7, 1.0, 0.5).into());
        let black_selected = materials.add(Color::rgba(0.4, 0.3, 0.6, 0.5).into());
        let border = materials.add(Color::rgba(0.5, 0.1, 0.1, 0.5).into());

        BoardColors {
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


/// Square on the chessboard
#[derive(Component, Copy, Clone, Debug)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}
/// returns whether a given square is white.
fn is_white(x: u8, y: u8) -> bool {
    (x + y + 1) % 2 == 0
}

/// Creates the checked pattern that is used on a chessboard.
/// Each of the squares can be selected or hovered over because it is assigned a pickablebundle.
/// The bottom left square is on position (0,0).
fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<BoardColors>,
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
                    material: if is_white(i, j) {
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

/// Highlights the hovered or selected square.
fn color_squares(
    mut query: Query<(&Square, &mut Handle<StandardMaterial>, &Hover, &Selection)>,
    colors: Res<BoardColors>,
) {
    for (square, mut handle, hover, selection) in query.iter_mut() {
        if selection.selected() {
            *handle = if is_white(square.x, square.y) {
                colors.white_selected.clone()
            } else {
                colors.black_selected.clone()
            }
        } else if hover.hovered() {
            *handle = if is_white(square.x, square.y) {
                colors.white_hovered.clone()
            } else {
                colors.black_hovered.clone()
            }
        }
    }
}

///Creates the border around the board, this board is a fixed rigid body and thus not affected by forces and gravity.
fn create_border(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<BoardColors>,
) {
    let mesh = meshes.add(Mesh::from(shape::Box {
        min_x: -2.5,
        max_x: 9.5,
        min_y: -0.25,
        max_y: -SMALL_FLOAT,
        min_z: -3.5,
        max_z: 10.5,
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
                .insert(Friction {
                    coefficient: BOARD_FRICTION,
                    combine_rule: CoefficientCombineRule::Max,
                })
                .insert(Transform::from_translation(BOARD_OFFSET));
        });
}
