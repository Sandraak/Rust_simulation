use bevy::prelude::*;
use bevy_mod_picking::{Hover, PickableBundle, Selection};

use crate::{chess::chess::Piece, pieces::PieceComponent};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SquareColors>()
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create_board)
            .add_startup_system(create_border)
            .add_system(color_squares)
            .add_system(select_square);
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
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.,
        ..default()
    }));

    for i in -1..9 {
        for j in -2..10 {
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: colors.border.clone(),
                    transform: Transform::from_translation(Vec3::new(i as f32, -0.01, j as f32)),
                    ..Default::default()
                })
                .insert(PickableBundle::default());
        }
    }
}

// /deze fucntie doet eigenlijk 3 dingen, opslitsen + betere naamgeving
fn select_square(
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
    info!("mouse pressed");

    for (square, interaction, entity) in square_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            let optional_piece = pieces_query
                .into_iter()
                .find(|piece| piece.0.x as u8 == square.x && piece.0.y as u8 == square.y);
            if optional_piece.is_some() {
                // selected piece is not really the optional_piece from the query but a copy of that piece.
                selected_piece.selected = Some(optional_piece.unwrap().1);
                info!("piece: {:?}", selected_piece.selected);
                info!("optional_piece: {:?}", optional_piece);
                return;
            }
        }
    }


    if selected_piece.selected.is_some() {
        for (square, interaction, entity) in square_query.iter_mut() {
            if let Interaction::Clicked = interaction {
                selected_square.selected = Some(*square);
                info!("selected square: {:?}", selected_square.selected);
            }
        }
    }
    if selected_piece.selected.is_some() && selected_square.selected.is_some() {

        info!("let's move");
        let (mut selected_piece_com, _) = pieces_query.get_mut(selected_piece.selected.unwrap()).unwrap();
       
        info!("selected_piece BEFORE move: {:?}", selected_piece);       
        info!("selected_piece_comp BEFORE move: {:?}", selected_piece_com);

        selected_piece_com.x = selected_square.selected.unwrap().x as usize;
        selected_piece_com.y = selected_square.selected.unwrap().y as usize;

    }
}

// fn find_piece_on_square_mut(pieces_query: &mut Query<&mut PieceComponent>, square: &Square) -> Option<Mut<PieceComponent>>{
//     pieces_query
//         .iter_mut()
//         .find(move |piece| piece.x as u8 == square.x && piece.y as u8 == square.y)
// }

// fn select_piece(
//     square_query: &mut Query< (&Square, &Interaction)>,
//     mut pieces_query: Query<&mut PieceComponent>,
// ) -> Option<Mut<PieceComponent>> {
//     // let mut selected_piece = None;
//     square_query.iter_mut().find_map(move |(square, interaction)| {
//         if let Interaction::Clicked = interaction {
//             pieces_query
//             .iter_mut()
//             .find(move |piece| piece.x as u8 == square.x && piece.y as u8 == square.y)
//         } else {
//             None
//         }
//     })
//     // for (square, interaction) in square_query.iter_mut() {
//     //     if let Interaction::Clicked = interaction {
//     //             selected_piece = pieces_query
//     //             .iter_mut()
//     //             .find(|piece| piece.x as u8 == square.x && piece.y as u8 == square.y);
//     //             info!("piece: {:?}", selected_piece);
//     //     }
//     // }
//     // selected_piece
// }

// fn select_square<'a, 'w, 's>(
//     square_query: &mut Query<'w, 's, (&'a Square, &'a Interaction)>,
//     selected_piece: &mut Option<Mut<'a, PieceComponent>>,
// ) -> Option<Square> where 'w: 's, {
//     let mut selected_square = None;
//     if selected_piece.is_some() {
//         for (square, interaction) in square_query.iter_mut() {
//             if let Interaction::Clicked = interaction {
//                 selected_square = Some(*square);
//                 info!("selected square: {:?}", selected_square);
//             }
//         }
//     }
//     selected_square
// }

// fn perform_move<'a, 'w, 's>(
//     mouse_button_inputs: Res<'w, Input<MouseButton>>,
//     mut square_query: Query<'w, 's, (&'a Square, &'a Interaction)>,
//     pieces_query: Query<'w, 's, &'a mut PieceComponent>,
// ) where 'w: 's, {
//     if !mouse_button_inputs.just_pressed(MouseButton::Left) {
//         return;
//     }

//     let mut selected_piece = select_piece(&mut square_query, pieces_query);
//     let mut selected_square = select_square(&mut square_query, &mut selected_piece);

//     if selected_square.is_some() && selected_piece.is_some() {
//         selected_piece.as_mut().unwrap().x = selected_square.unwrap().x as usize;
//         selected_piece.as_mut().unwrap().y = selected_square.unwrap().y as usize;
//         selected_piece = None;
//         selected_square = None;
//     }
// }

//algemene tip, meteen beginnen met comments plaatsen
