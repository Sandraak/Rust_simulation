use bevy::prelude::{EventReader, EventWriter, Plugin, ResMut, Res};

use crate::{
    chess::{chess::Chess, chess::Move},
    controller::controller::{ComputerTurnEvent, CurrentMove, MoveEvent, Player, PlayerTurn},
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

/// When a new [`ComputerTurnEvent`] is registered this function will generate a new move,
/// if it's the computer player's turn. When a new move had been found, this move will be 
/// stored in [`CurrentMove`]
/// and the function will send a [`MoveEvent`] triggering [`update_path`]
/// When there are no more moves, the game has ended and 
/// the outcome will be printed.
/// 
/// [`update_path`]: crate::controller::controller::update_path
pub fn return_move(
    mut computer_turn: EventReader<ComputerTurnEvent>,
    boardstate: Res<BoardState>,
    player_turn: Res<PlayerTurn>,
    mut new_move: EventWriter<MoveEvent>,
    mut current_move: ResMut<CurrentMove>,
) {
    for _event in computer_turn.iter() {
        if player_turn.turn == Player::Computer {
            let chess = boardstate.chess;
            let best_move = minimax(&chess, 4, i16::MIN, i16::MAX);
            if best_move.m.is_some() {
                current_move.current_move = best_move.m.unwrap();
                new_move.send(MoveEvent);
            } else {
                match chess.outcome().unwrap() {
                    Outcome::Winner(color) => println!("{color} wins!"),
                    Outcome::Stalemate => println!("it's a stalemate!"),
                }
            }
        }
    }
}
/// Function for determining the next move of the computer player.
/// It takes the current [`BoardState`] and checks what is the best move
/// for the current player, Black or White. Alpha-beta pruning is used
/// to scrap the irrelevant branches, making the function a lot faster.
/// The depth states how many moves deep the algorithm searches for the best move.
/// It's a depth first search recursive function, that first checks a series of moves until the given depth,
/// calculates a score for the state at the end of these moves. Then it moves on to the next series of possible moves.
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
