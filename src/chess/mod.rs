use bevy::prelude::*;

use self::chess::Chess;

pub mod chess;
pub mod pos;

#[derive(Resource, Default)]
pub struct BoardState {
    pub chess: Chess,
}
