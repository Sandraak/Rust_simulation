use bevy::prelude::{EventReader, EventWriter, Plugin, Res, ResMut};

use crate::{
    chess::{chess::Chess, chess::Move},
    controller::controller::{self, ComputerTurnEvent, CurrentMove, MoveEvent, Player, PlayerTurn},
};

use super::{
    chess::{Color, Outcome},
    BoardState,
};
pub struct BestMove {
    pub m: Option<Move>,
    score: i16,
}

pub struct ChessComputerPlugin;

impl Plugin for ChessComputerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(return_move);
    }
}

pub fn return_move(
    mut computer_turn: EventReader<ComputerTurnEvent>,
    boardstate: ResMut<BoardState>,
    player_turn: ResMut<PlayerTurn>,
    mut new_move: EventWriter<MoveEvent>,
    mut current_move: ResMut<CurrentMove>,
) {
    for _event in computer_turn.iter() {
        if player_turn.turn == Player::Computer {
            let chess = boardstate.chess;
            let best_move = minimax(&chess, 3, i16::MIN, i16::MAX);
            if best_move.m.is_some() {
                current_move.current_move = best_move.m.unwrap();
                new_move.send(MoveEvent);
                println!("move event send: Move : {:?}", current_move.current_move);
            } else {
                match chess.outcome().unwrap() {
                    Outcome::Winner(color) => println!("{color} wins!"),
                    Outcome::Stalemate => println!("it's a stalemate!"),
                }
            }
        }
    }
}

pub fn minimax(chess: &Chess, depth: u8, mut alpha: i16, mut beta: i16) -> BestMove {
    if (depth == 0) || chess.outcome().is_some() {
        BestMove {
            m: None,
            score: chess.evaluate(),
        }
    } else if chess.turn == Color::White {
        let mut best_move = None;
        let mut best_score = i16::MIN;

        for m in chess.moves() {
            let mut copy = *chess;
            copy.perform(m);
            let score = minimax(&copy, depth - 1, alpha, beta).score;
            if score > best_score || best_move.is_none() {
                best_score = score;
                best_move = Some(m);
                if score > alpha {
                    alpha = score;
                }
                if alpha >= beta {
                    break;
                }
            }
        }

        BestMove {
            m: Some(best_move.unwrap()),
            score: best_score,
        }
    } else {
        let mut best_move = None;
        let mut best_score = i16::MAX;

        for m in chess.moves() {
            let mut copy = *chess;
            copy.perform(m);
            let score = minimax(&copy, depth - 1, alpha, beta).score;
            if score < best_score || best_move.is_none() {
                best_score = score;
                best_move = Some(m);
                if score < beta {
                    beta = score;
                }
                if alpha >= beta {
                    break;
                }
            }
        }

        BestMove {
            m: Some(best_move.unwrap()),
            score: best_score,
        }
    }
}
