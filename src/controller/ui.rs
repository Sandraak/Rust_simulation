use super::controller::{self, CurrentMove, MoveEvent, Player, PlayerTurn};
use crate::{
    chess::{chess::Move, pos::Pos},
    simulation::{board::Square, pieces::PieceComponent},
};
use bevy::prelude::*;
/// Plugin initilizing the resources and running the systems for the bevy app.
pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_system(perform_move);
    }
}

/// Resource containing the currently selected square.
#[derive(Default, Resource)]
struct SelectedSquare {
    selected: Option<Square>,
}

/// Resource containing the currently selected  piece.
#[derive(Default, Resource, Debug)]
struct SelectedPiece {
    selected: Option<Entity>,
}

/// Allows the human player to move a piece to an empty square by clicking with the left mouse button
/// on the piece and desired location. Sends a [`MoveEvent`], which triggers [`update_path`] in controller.rs
///
/// [`update_path`]: super::controller::update_path
fn perform_move(
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut square_query: Query<(&Square, &Interaction)>,
    mut pieces_query: Query<(&mut PieceComponent, Entity)>,
    mut new_move: EventWriter<MoveEvent>,
    mut current_move: ResMut<CurrentMove>,
    player_turn: Res<PlayerTurn>,
) {
    if player_turn.turn == Player::Human {
        if !mouse_button_inputs.just_pressed(MouseButton::Left) {
            return;
        }
        //selects the piece that was clicked on
        if selected_piece.selected.is_none() {
            for (square, interaction) in square_query.iter_mut() {
                if let Interaction::Clicked = interaction {
                    let optional_piece = pieces_query.into_iter().find(|piece| {
                        piece.0.target_x as u8 == square.x && piece.0.target_y as u8 == square.y
                    });
                    if optional_piece.is_some() {
                        if optional_piece
                            .filter(|piece| piece.0.piece.color == player_turn.color)
                            .is_some()
                        {
                            // Add the identifier of the piece entity to selected_piece. This identifier is later used to query the location of the selected piece.
                            selected_piece.selected = Some(optional_piece.unwrap().1);
                            //return so that the selected square won't be the same as the square the selected piece is on.
                            return;
                        }
                    }
                }
            }
        }
        // When a piece is selected, selects a square to where the selected piece will move.
        if selected_piece.selected.is_some() {
            for (square, interaction) in square_query.iter_mut() {
                if let Interaction::Clicked = interaction {
                    selected_square.selected = Some(*square);
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
                        x: selected_piece_com.target_y as isize,
                        y: selected_piece_com.target_x as isize,
                    },
                    to: Pos {
                        x: selected_square.selected.unwrap().y as isize,
                        y: selected_square.selected.unwrap().x as isize,
                    },
                },
            };
            new_move.send(MoveEvent);
            selected_piece_com.target_x = selected_square.selected.unwrap().x as usize;
            selected_piece_com.target_y = selected_square.selected.unwrap().y as usize;

            selected_piece.selected = None;
            selected_square.selected = None;
        }
    }
}
