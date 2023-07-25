use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::{Index, IndexMut, Not};

use super::pos::*;

const TEST_BOARD_1: [[std::option::Option<Piece>; 8]; 8] =  [
    [
        Some(Piece::WHITE_ROOK),
        Some(Piece::WHITE_KNIGHT),
        Some(Piece::WHITE_BISHOP),
        Some(Piece::WHITE_QUEEN),
        Some(Piece::WHITE_KING),
        Some(Piece::WHITE_BISHOP),
        Some(Piece::WHITE_KNIGHT),
        Some(Piece::WHITE_ROOK),
    ],
    [
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::BLACK_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
    ],
    [
        None,
        None,
        None,
        Some(Piece::BLACK_PAWN),
        Some(Piece::BLACK_KNIGHT),
        Some(Piece::BLACK_PAWN),
        None,
        None,
    ],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [
        Some(Piece::BLACK_PAWN),
        Some(Piece::BLACK_PAWN),
        Some(Piece::BLACK_PAWN),
        None,
        None,
        None,
        Some(Piece::BLACK_PAWN),
        Some(Piece::BLACK_PAWN),
    ],
    [
        Some(Piece::BLACK_ROOK),
        Some(Piece::BLACK_KNIGHT),
        Some(Piece::BLACK_BISHOP),
        Some(Piece::BLACK_QUEEN),
        Some(Piece::BLACK_KING),
        Some(Piece::BLACK_BISHOP),
        None,
        Some(Piece::BLACK_ROOK),
    ],
];

const TEST_BOARD_2: [[std::option::Option<Piece>; 8]; 8] =  [
    [
        Some(Piece::WHITE_ROOK),
        Some(Piece::WHITE_KNIGHT),
        Some(Piece::WHITE_BISHOP),
        Some(Piece::WHITE_QUEEN),
        Some(Piece::WHITE_KING),
        Some(Piece::WHITE_BISHOP),
        Some(Piece::WHITE_KNIGHT),
        Some(Piece::WHITE_ROOK),
    ],
    [
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
    ],
    [
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
        Some(Piece::WHITE_PAWN),
    ],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [
        Some(Piece::BLACK_PAWN),
        Some(Piece::BLACK_PAWN),
        Some(Piece::BLACK_PAWN),
        Some(Piece::BLACK_PAWN),
        Some(Piece::BLACK_PAWN),
        Some(Piece::BLACK_PAWN),
        Some(Piece::BLACK_PAWN),
        Some(Piece::BLACK_PAWN),
    ],
    [
        Some(Piece::BLACK_ROOK),
        Some(Piece::BLACK_KNIGHT),
        Some(Piece::BLACK_BISHOP),
        Some(Piece::BLACK_QUEEN),
        Some(Piece::BLACK_KING),
        Some(Piece::BLACK_BISHOP),
        Some(Piece::BLACK_KNIGHT),
        Some(Piece::BLACK_ROOK),
    ],
];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Graveyard {
    pub graveyard: [[Option<Piece>; 2]; 8],
}

impl Graveyard {
    pub fn default() -> Self {
        let graveyard = [
            [None, None],
            [None, None],
            [None, None],
            [None, None],
            [None, None],
            [None, None],
            [None, None],
            [None, None],
        ];
        Graveyard { graveyard }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Chess {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Color,
    /// Keeps track of the current positions of both kings. White's king's position is stored on
    /// index 0 and black's on 1.
    pub kings: [Pos; 2],
    pub graveyards: [Graveyard; 2],
}

///Board staat nu verkeerd om, ff omdraaien.
impl Chess {
    pub fn new() -> Self {
        let board = TEST_BOARD_1;
        // [
        //     [
        //         Some(Piece::WHITE_ROOK),
        //         Some(Piece::WHITE_KNIGHT),
        //         Some(Piece::WHITE_BISHOP),
        //         Some(Piece::WHITE_QUEEN),
        //         Some(Piece::WHITE_KING),
        //         Some(Piece::WHITE_BISHOP),
        //         Some(Piece::WHITE_KNIGHT),
        //         Some(Piece::WHITE_ROOK),
        //     ],
        //     [
        //         Some(Piece::WHITE_PAWN),
        //         Some(Piece::WHITE_PAWN),
        //         Some(Piece::WHITE_PAWN),
        //         Some(Piece::WHITE_PAWN),
        //         Some(Piece::BLACK_PAWN),
        //         Some(Piece::WHITE_PAWN),
        //         Some(Piece::WHITE_PAWN),
        //         Some(Piece::WHITE_PAWN),
        //     ],
        //     [
        //         None,
        //         None,
        //         None,
        //         Some(Piece::BLACK_PAWN),
        //         Some(Piece::BLACK_KNIGHT),
        //         Some(Piece::BLACK_PAWN),
        //         None,
        //         None,
        //     ],
        //     [None, None, None, None, None, None, None, None],
        //     [None, None, None, None, None, None, None, None],
        //     [None, None, None, None, None, None, None, None],
        //     [
        //         Some(Piece::BLACK_PAWN),
        //         Some(Piece::BLACK_PAWN),
        //         Some(Piece::BLACK_PAWN),
        //         None,
        //         None,
        //         None,
        //         Some(Piece::BLACK_PAWN),
        //         Some(Piece::BLACK_PAWN),
        //     ],
        //     [
        //         Some(Piece::BLACK_ROOK),
        //         Some(Piece::BLACK_KNIGHT),
        //         Some(Piece::BLACK_BISHOP),
        //         Some(Piece::BLACK_QUEEN),
        //         Some(Piece::BLACK_KING),
        //         Some(Piece::BLACK_BISHOP),
        //         None,
        //         Some(Piece::BLACK_ROOK),
        //     ],
        // ];
        let turn = Color::default();
        let kings = [Pos::new(4, 7), Pos::new(4, 0)];
        let graveyards = [Graveyard::default(), Graveyard::default()];

        Chess {
            board,
            turn,
            kings,
            graveyards,
        }
    }

    /// Checks whether a given position is on the board.
    fn on_board(pos: &Pos) -> bool {
        (0 <= pos.x() && pos.x() < 8) && (0 <= pos.y() && pos.y() < 8)
    }

    /// Returns an iterator over all positions of a chess board.
    pub fn board_positions() -> impl Iterator<Item = Pos> {
        (0..8).flat_map(|x| (0..8).map(move |y| Pos::new(x, y)))
    }

    pub fn graveyard_positions(&self) -> impl Iterator<Item = Pos> {
        let mut start = 9;
        let mut end = 10;
        if self.turn == Color::White {
            start = -3;
            end = -2;
        }
        (start..=end).flat_map(|x| (-1..=8).map(move |y| Pos::new(x, y)))
    }

    // /// Returns an iterator over all pieces on the board.
    // fn pieces(&self) -> impl Iterator<Item = (Pos, Piece)> + '_ {
    //     Self::board_positions().filter_map(|pos| self[pos].map(|piece| (pos, piece)))
    // }
}

impl Default for Chess {
    fn default() -> Self {
        Chess::new()
    }
}

impl<P> Index<P> for Chess
where
    P: Borrow<Pos>,
{
    type Output = Option<Piece>;

    fn index(&self, index: P) -> &Self::Output {
        let pos = index.borrow();
        if Self::on_board(pos) {
            &self.board[pos.y() as usize][pos.x() as usize]
        } else {
            &None
        }
    }
}

impl<P> IndexMut<P> for Chess
where
    P: Borrow<Pos>,
{
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        &mut self.board[index.borrow().y() as usize][index.borrow().x() as usize]
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Piece {
    pub color: Color,
    pub kind: Kind,
}

impl Piece {
    pub const WHITE_PAWN: Piece = Piece::new(Color::White, Kind::Pawn);
    pub const WHITE_ROOK: Piece = Piece::new(Color::White, Kind::Rook);
    pub const WHITE_KNIGHT: Piece = Piece::new(Color::White, Kind::Knight);
    pub const WHITE_BISHOP: Piece = Piece::new(Color::White, Kind::Bishop);
    pub const WHITE_QUEEN: Piece = Piece::new(Color::White, Kind::Queen);
    pub const WHITE_KING: Piece = Piece::new(Color::White, Kind::King);

    pub const BLACK_PAWN: Piece = Piece::new(Color::Black, Kind::Pawn);
    pub const BLACK_ROOK: Piece = Piece::new(Color::Black, Kind::Rook);
    pub const BLACK_KNIGHT: Piece = Piece::new(Color::Black, Kind::Knight);
    pub const BLACK_BISHOP: Piece = Piece::new(Color::Black, Kind::Bishop);
    pub const BLACK_QUEEN: Piece = Piece::new(Color::Black, Kind::Queen);
    pub const BLACK_KING: Piece = Piece::new(Color::Black, Kind::King);

    const fn new(color: Color, kind: Kind) -> Self {
        Piece { color, kind }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Color {
    Black,
    #[default]
    White,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Kind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Move {
    pub from: Pos,
    pub to: Pos,
}

