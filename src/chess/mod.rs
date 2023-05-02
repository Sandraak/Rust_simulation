use bevy::prelude::*;

use self::chess::Chess;

pub mod chess;
mod pos;

#[derive(Resource, Default)]
pub struct BoardState{
chess: Chess
}