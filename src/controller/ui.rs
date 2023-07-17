use super::controller::{self, CurrentMove, MoveEvent, PlayerTurn};
use crate::{
    chess::{chess::Move, pos::Pos},
    simulation::{board::Square, pieces::PieceComponent},
};
use bevy::prelude::*;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_system(perform_move);
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
// /Allows the user to move a piece to an empty square by clicking on the piece and desired location.
fn perform_move(
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut square_query: Query<(&Square, &Interaction)>,
    mut pieces_query: Query<(&mut PieceComponent, Entity)>,
    mut new_move: EventWriter<MoveEvent>,
    mut current_move: ResMut<CurrentMove>,
    mut player_turn: ResMut<PlayerTurn>,
    // mut magnet_query: Query<&mut Magnet>,
) {
    if !player_turn.turn {
        if !mouse_button_inputs.just_pressed(MouseButton::Left) {
            return;
        }
        //selects the piece that was clicked on
        for (square, interaction) in square_query.iter_mut() {
            if let Interaction::Clicked = interaction {
                let optional_piece = pieces_query.into_iter().find(|piece| {
                    piece.0.target_x as u8 == square.x && piece.0.target_y as u8 == square.y
                });
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
            for (square, interaction) in square_query.iter_mut() {
                if let Interaction::Clicked = interaction {
                    selected_square.selected = Some(*square);
                    info!("selected square: {:?}", selected_square.selected);
                }
            }
        }
        // Move the selected piece to the selected square.
        if selected_piece.selected.is_some() && selected_square.selected.is_some() {
            // Get the PieceComponent of the piece with the identifier that was specified earlier.
            let (mut selected_piece_com, _) = pieces_query
                .get_mut(selected_piece.selected.unwrap())
                .unwrap();

            *current_move = controller::CurrentMove {
                current_move: Move {
                    from: Pos {
                        x: selected_piece_com.target_x as isize,
                        y: selected_piece_com.target_y as isize,
                    },
                    to: Pos {
                        x: selected_square.selected.unwrap().x as isize,
                        y: selected_square.selected.unwrap().y as isize,
                    },
                },
            };
            new_move.send(MoveEvent);
            println!("move event send: Move : {:?}", current_move.current_move);

            selected_piece_com.target_x = selected_square.selected.unwrap().x as usize;
            selected_piece_com.target_y = selected_square.selected.unwrap().y as usize;

            selected_piece.selected = None;
            selected_square.selected = None;
        }
    }
}
