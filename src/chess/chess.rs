use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::{Index, IndexMut, Not};

use super::pos::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Chess {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Color,
    /// Keeps track of the current positions of both kings. White's king's position is stored on
    /// index 0 and black's on 1.
    pub kings: [Pos; 2],
}

///Board staat nu verkeerd om, ff omdraaien.
impl Chess {
    pub fn new() -> Self {
        let board = [
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
                Some(Piece::BLACK_PAWN),
                Some(Piece::BLACK_PAWN),
                Some(Piece::BLACK_PAWN),
                Some(Piece::BLACK_PAWN),
                Some(Piece::BLACK_PAWN),
                Some(Piece::BLACK_PAWN),
                Some(Piece::BLACK_PAWN),
                Some(Piece::BLACK_PAWN),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
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
                Some(Piece::WHITE_ROOK),
                Some(Piece::WHITE_KNIGHT),
                Some(Piece::WHITE_BISHOP),
                Some(Piece::WHITE_QUEEN),
                Some(Piece::WHITE_KING),
                Some(Piece::WHITE_BISHOP),
                Some(Piece::WHITE_KNIGHT),
                Some(Piece::WHITE_ROOK),
            ],
        ];

        let turn = Color::default();

        let kings = [Pos::new(4, 7), Pos::new(4, 0)];

        Chess { board, turn, kings }
    }

    /// Checks whether a given position is on the board.
    fn on_board(pos: &Pos) -> bool {
        (0 <= pos.x() && pos.x() < 8) && (0 <= pos.y() && pos.y() < 8)
    }

    /// Returns an iterator over all positions of a chess board.
    pub fn board_positions() -> impl Iterator<Item = Pos> {
        (0..8).flat_map(|x| (0..8).map(move |y| Pos::new(x, y)))
    }

    /// Returns an iterator over all pieces on the board.
    fn pieces(&self) -> impl Iterator<Item = (Pos, Piece)> + '_ {
        Self::board_positions().filter_map(|pos| self[pos].map(|piece| (pos, piece)))
    }

    /// Generates all legal moves for the current player.
    pub fn moves(&self) -> impl Iterator<Item = Move> + '_ {
        self.unsafe_moves(self.turn).filter(|m| self.is_safe(*m))
    }

    /// Checks whether performing a move does not check the current player's own king.
    fn is_safe(&self, m: Move) -> bool {
        let mut copy = *self;
        copy.perform(m);
        !copy.is_checked(self.turn)
    }

    /// Generates all moves for the given player, even those that would check their own king.
    /// NEEDS BETTER COMMENTS
    fn unsafe_moves(&self, player: Color) -> impl Iterator<Item = Move> + '_ {
        self.pieces()
            .filter(move |(_, piece)| piece.color == player)
            .flat_map(move |(from, piece)| match piece.kind {
                Kind::Pawn => {
                    let (step, captures, start_row) = match player {
                        Color::Black => (Shift::DOWN, vec![Shift::DOWN_RIGHT, Shift::DOWN_LEFT], 1),
                        Color::White => (Shift::UP, vec![Shift::UP_LEFT, Shift::UP_RIGHT], 6),
                    };

                    let captures = captures
                        .into_iter()
                        .map(move |dir| from + dir)
                        .filter(move |to| {
                            self[to]
                                .as_ref()
                                .map(|piece| piece.color != player)
                                .unwrap_or_default()
                        })
                        .map(move |to| Move::new(from, to));

                    let to = from + step;
                    let too = from + step * 2;

                    let step =
                        (self[to].is_none() && Self::on_board(&to)).then(|| Move::new(from, to));
                    let leap = (from.y() == start_row && self[to].is_none() && self[too].is_none())
                        .then(|| Move::new(from, too));

                    Box::new(captures.chain(leap).chain(step)) as Box<dyn Iterator<Item = Move>>
                }
                Kind::Rook => Box::new(Shift::CARDINAL_DIRS.iter().flat_map(move |dir| {
                    let mut capture = false;
                    (1..)
                        .map(move |distance| from + *dir * distance)
                        .take_while(Self::on_board)
                        .take_while(move |to| self.is_traversable(player, to, &mut capture))
                        .map(move |to| Move::new(from, to))
                })),
                Kind::Knight => Box::new(
                    Shift::JUMPS
                        .iter()
                        .map(move |jump| from + *jump)
                        .filter(Self::on_board)
                        .filter(move |to| {
                            self[to]
                                .as_ref()
                                .map(|piece| piece.color != player)
                                .unwrap_or(true)
                        })
                        .map(move |to| Move::new(from, to)),
                ),
                Kind::Bishop => Box::new(Shift::DIAGONAL_DIRS.iter().flat_map(move |dir| {
                    let mut capture = false;
                    (1..)
                        .map(move |distance| from + *dir * distance)
                        .take_while(Self::on_board)
                        .take_while(move |to| self.is_traversable(player, to, &mut capture))
                        .map(move |to| Move::new(from, to))
                })),
                Kind::Queen => Box::new(Shift::DIRS.iter().flat_map(move |dir| {
                    let mut capture = false;
                    (1..)
                        .map(move |distance| from + *dir * distance)
                        .take_while(Self::on_board)
                        .take_while(move |to| self.is_traversable(player, to, &mut capture))
                        .map(move |to| Move::new(from, to))
                })),
                Kind::King => Box::new(
                    Shift::DIRS
                        .iter()
                        .map(move |dir| from + *dir)
                        .filter(Self::on_board)
                        .filter(move |to| {
                            self[to]
                                .as_ref()
                                .map(|piece| piece.color != player)
                                .unwrap_or(true)
                        })
                        .map(move |to| Move::new(from, to)),
                ),
            })
    }

    /// Performs a move, changing the board state.
    pub fn perform(&mut self, m: Move) {
        if self[m.from].unwrap().kind == Kind::King {
            self.kings[self.turn.king_index()] = m.to;
        }
        self[m.to] = self[m.from].take();
        self.turn = !self.turn;
    }

    /// Evaluates how many "points" a board state is worth. A positive score indicates that white is
    /// in a favorable position, and a negative score indicates that black is currently better off.
    pub fn evaluate(&self) -> i16 {
        match self.outcome() {
            None => self.pieces().map(|(_, piece)| piece.base_value()).sum(),
            Some(outcome) => outcome.value(),
        }
    }

    /// For pieces that can move entire rows, lanes, or diagonals, this function checks whether they
    /// can continue moving in a straight line.
    ///
    /// Pieces can move as long a tiles are empty, but must stop immediately upon encountering a
    /// piece of the same color. They can continue if they encounter a piece of the opposite color
    /// to capture it, but cannot continue after that. This is what the `capture` variable is used
    /// for; it should be set to false at the start of the row, lane, or diagonal, and this function
    /// will set it to true as soon as a piece of the opposite color is detected, stopping on the
    /// next tile.
    ///
    /// Note that this function does not check whether the destination is on the board or not.
    /// [`on_board`] can be used first for that purpose.
    ///
    /// [`on_board`]: #method.on_board
    fn is_traversable(&self, player: Color, to: &Pos, capture: &mut bool) -> bool {
        if *capture {
            false
        } else {
            match &self[to] {
                None => true,
                Some(piece) => {
                    if piece.color == player {
                        false
                    } else {
                        *capture = true;
                        true
                    }
                }
            }
        }
    }

    /// Checks whether the given player is currently checked.
    fn is_checked(&self, player: Color) -> bool {
        let king = self.kings[player.king_index()];
        self.unsafe_moves(!player).any(|m| m.to == king)
    }

    /// Returns the outcome of the game state. A `None` output indicates that the game is not over,
    /// whereas `Some(Outcome)` indicates which player has won the game, or if there was a
    /// stalemate.
    pub fn outcome(&self) -> Option<Outcome> {
        if self.moves().next().is_none() {
            // No legal moves for the current player, the game is over
            if self.is_checked(self.turn) {
                // The current player is checked, so the other player wins
                Some(Outcome::Winner(!self.turn))
            } else {
                // The current player is not checked, so it's a stalemate
                Some(Outcome::Stalemate)
            }
        } else {
            // There are moves left for the current player, so the game is not over yet
            None
        }
    }
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
pub enum Outcome {
    Winner(Color),
    Stalemate,
}

impl Outcome {
    pub fn value(&self) -> i16 {
        match self {
            Outcome::Winner(color) => match color {
                Color::Black => i16::MIN,
                Color::White => i16::MAX,
            },
            Outcome::Stalemate => 0,
        }
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

    pub fn base_value(&self) -> i16 {
        match self.color {
            Color::Black => -self.kind.base_value(),
            Color::White => self.kind.base_value(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Color {
    Black,
    #[default]
    White,
}

impl Color {
    /// Checks whether a new game state evaluation is better than the previous best for the current
    /// player.
    pub fn improves(&self, score: i16, best_score: Option<i16>) -> bool {
        match best_score {
            None => true,
            Some(best) => match self {
                Color::Black => score < best,
                Color::White => score > best,
            },
        }
    }

    fn king_index(&self) -> usize {
        match self {
            Color::Black => 1,
            Color::White => 0,
        }
    }
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

impl Kind {
    pub fn base_value(&self) -> i16 {
        match self {
            Kind::Pawn => 1,
            Kind::Rook => 5,
            Kind::Knight => 3,
            Kind::Bishop => 3,
            Kind::Queen => 9,
            Kind::King => 0,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Move {
    from: Pos,
    to: Pos,
}

impl Move {
    fn new(from: Pos, to: Pos) -> Self {
        Move { from, to }
    }
}
