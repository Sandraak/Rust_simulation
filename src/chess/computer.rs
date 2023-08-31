use bevy::prelude::{EventReader, EventWriter, Plugin, Res, ResMut};

use crate::{
    chess::{chess::Chess, chess::Move},
    controller::controller::{ComputerTurnEvent, CurrentMove, MoveEvent, Player, PlayerTurn},
};

use super::{
    chess::{Color, Outcome},
    BoardState,
};
/// Move with the highest score acording to the minimax algorithm.
pub struct BestMove {
    pub m: Option<Move>,
    score: i16,
}
/// Plugin that runs the system for the bevy app.
pub struct ChessComputerPlugin;

impl Plugin for ChessComputerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(return_move);
    }
}

/// When a new [`ComputerTurnEvent`] is registered this function will generate a new move,
/// if it's the computer player's turn. When a new move had been found, this move will be
/// stored in [`CurrentMove`]
/// and the function will send a [`MoveEvent`] triggering [`update_path`].
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
            let best_move = minimax(&chess, 3, i16::MIN, i16::MAX);
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
/// Function for determining the next move of the computer player. For the Black player
/// the score has to be Minimized, and maximized for the white player
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

//TESTS
#[cfg(test)]
mod tests {
    use crate::chess::{
        chess::Chess,
        chess::{Color, Move},
        computer::minimax,
        pos::Pos,
    };

    #[test]
    fn test_minimax() {
        // Create a chess board with a specific state for testing
        let mut chess = Chess::default();
        // Setup for fools mate
        chess.perform(Move {
            from: Pos::new(5, 1),
            to: Pos::new(5, 2),
        });
        chess.perform(Move {
            from: Pos::new(4, 6),
            to: Pos::new(4, 5),
        });
        chess.perform(Move {
            from: Pos::new(6, 1),
            to: Pos::new(6, 3),
        });
        // The black player can checkmate white by performing:
        // chess.perform(Move::new(Pos::new(3, 7), Pos::new(7, 3)));
        chess.turn = Color::Black;
        // Call the minimax function with the known board state
        let best_move = minimax(&chess, 2, i16::MIN, i16::MAX);
        // Assert that the best move and score match the expected values
        // In this example, we expect the best move to be the one that puts white in a checkmate
        assert_eq!(
            best_move.m.unwrap(),
            Move {
                from: Pos::new(3, 7),
                to: Pos::new(7, 3)
            }
        );
    }
}
