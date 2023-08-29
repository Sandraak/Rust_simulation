use self::chess::Chess;
use bevy::prelude::*;

/// Handles the boardstate and the rules of chess.
pub mod chess;
/// Chess computer
pub mod computer;
/// Module that makes it easy to deal with positions on the board
pub mod pos;

///Resource variant of [`Chess`]
#[derive(Resource, Default)]
pub struct BoardState {
    pub chess: Chess,
}
