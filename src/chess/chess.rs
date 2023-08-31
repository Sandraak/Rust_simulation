use crate::chess::pos::{Pos, Shift};
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut, Not};

/// When a piece has been captured it will move to the sidelines,
/// which are 2 colums named the Graveyard.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Graveyard {
    pub graveyard: [[Option<Piece>; 1]; 8],
}

impl Graveyard {
    pub fn default() -> Self {
        let graveyard = [
            [None],
            [None],
            [None],
            [None],
            [None],
            [None],
            [None],
            [None],
        ];
        Graveyard { graveyard }
    }
}

/// Keeps track of the state of the game, the location of all the pieces(boardstate) is kept in board. Whose turn it is in turn,
/// the location of both kings in kings. And the state of the graveyards in graveyards.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Chess {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Color,
    /// Keeps track of the current positions of both kings. White's king's position is stored on
    /// index 0 and black's on 1.
    pub kings: [Pos; 2],
    pub graveyards: [Graveyard; 2],
}

impl Chess {
    pub fn new() -> Self {
        let board = [
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
            [None, None, None, None, None, None, None, None],
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

        let turn = Color::default();

        let kings = [Pos::new(4, 0), Pos::new(4, 7)];
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
    /// Returns an iterator over all positions of a graveyard of the current player.
    pub fn graveyard_positions(&self) -> impl Iterator<Item = Pos> {
        let mut start = 9;
        let mut end = 9;
        if self.turn == Color::White {
            start = -2;
            end = -2;
        }
        (start..=end).flat_map(|x| (-1..=8).map(move |y| Pos::new(x, y)))
    }
    /// Returns an iterator over all positions around the board.
    pub fn border_positions() -> impl Iterator<Item = Pos> {
        let x_vec: [isize; 2] = [-1, 8];
        let y_vec: [isize; 2] = [-1, 8];
        (x_vec)
            .into_iter()
            .flat_map(|x| (-1..=8).map(move |y| Pos::new(x, y)))
            .chain(
                (y_vec)
                    .into_iter()
                    .flat_map(|y| (0..=7).map(move |x| Pos::new(x, y))),
            )
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
    /// Each kind of piece has their own set of moves, the function starts by filtering out
    /// all pieces that are not of the current players color. After that all moves are  
    fn unsafe_moves(&self, player: Color) -> impl Iterator<Item = Move> + '_ {
        self.pieces()
            .filter(move |(_, piece)| piece.color == player)
            .flat_map(move |(from, piece)| match piece.kind {
                Kind::Pawn => {
                    // A pawn can move one step straight towards the other side of the board, or diagonal
                    // when capturing an enemy piece
                    let (step, captures, start_row) = match player {
                        Color::Black => (Shift::UP, vec![Shift::UP_LEFT, Shift::UP_RIGHT], 6),
                        Color::White => (Shift::DOWN, vec![Shift::DOWN_RIGHT, Shift::DOWN_LEFT], 1),
                    };
                    // A pawn can only capture  an enemy piece that is diagonally in front it.
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
                    // a pawn can move one step towards the other side of the board
                    let step =
                        (self[to].is_none() && Self::on_board(&to)).then(|| Move::new(from, to));
                    // Or two steps when it is on its starting position
                    let leap = (from.y() == start_row && self[to].is_none() && self[too].is_none())
                        .then(|| Move::new(from, too));

                    Box::new(captures.chain(leap).chain(step)) as Box<dyn Iterator<Item = Move>>
                }
                Kind::Rook => Box::new(Shift::CARDINAL_DIRS.iter().flat_map(move |dir| {
                    //A rook can move straight over all traversable squares in a straight line across the board.
                    //It can capture an enemy piece when a straight path is clear between the rook and the piece.
                    let mut capture = false;
                    (1..)
                        .map(move |distance| from + *dir * distance)
                        .take_while(Self::on_board)
                        .take_while(move |to| self.is_traversable(player, to, &mut capture))
                        .map(move |to| Move::new(from, to))
                })),
                Kind::Knight => Box::new(
                    // A knight moves in L shaped jumps in all directions. It can jump over other friendly or enemy pieces
                    // It can capture an enemy piece when it jumps on it
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
                    //A bishop can move diagonally over all traversable squares in a straight line across the board.
                    //It can capture an enemy piece when a straight path is clear between the rook and the piece.
                    let mut capture = false;
                    (1..)
                        .map(move |distance| from + *dir * distance)
                        .take_while(Self::on_board)
                        .take_while(move |to| self.is_traversable(player, to, &mut capture))
                        .map(move |to| Move::new(from, to))
                })),
                Kind::Queen => Box::new(Shift::DIRS.iter().flat_map(move |dir| {
                    //The queen can move over all traversable squares in all directions.
                    let mut capture = false;
                    (1..)
                        .map(move |distance| from + *dir * distance)
                        .take_while(Self::on_board)
                        .take_while(move |to| self.is_traversable(player, to, &mut capture))
                        .map(move |to| Move::new(from, to))
                })),
                Kind::King => Box::new(
                    //The king can move one square in any direction.
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

///Winner or Stalemate
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

/// Information about the color and kind of the piece.
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

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Black => write!(f, "Black"),
            Color::White => write!(f, "White"),
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

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Pawn => write!(f, "♟︎"),
            Kind::Rook => write!(f, "♜"),
            Kind::Knight => write!(f, "♞"),
            Kind::Bishop => write!(f, "♝"),
            Kind::Queen => write!(f, "♛"),
            Kind::King => write!(f, "♚"),
        }
    }
}

/// Move from a position to a position.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Move {
    pub from: Pos,
    pub to: Pos,
}

impl Move {
    fn new(from: Pos, to: Pos) -> Self {
        Move { from, to }
    }
}

//TESTS
#[cfg(test)]
mod tests {
    use crate::chess::chess::*;
    #[test]
    fn test_default_graveyard() {
        let graveyard = Graveyard::default();
        // Verify that all elements in the graveyard are None
        for row in &graveyard.graveyard {
            for piece in row {
                assert_eq!(*piece, None);
            }
        }
    }

    #[test]
    fn test_default_chess() {
        let chess = Chess::default();

        // Verify the initial state of the chessboard
        assert_eq!(chess.turn, Color::White);
        assert_eq!(chess.kings[0], Pos::new(4, 0));
        assert_eq!(chess.kings[1], Pos::new(4, 7));

        // Verify that the initial positions of some pieces are correct
        assert_eq!(chess.board[0][0], Some(Piece::WHITE_ROOK));
        assert_eq!(chess.board[0][4], Some(Piece::WHITE_KING));
        assert_eq!(chess.board[7][7], Some(Piece::BLACK_ROOK));

        // Verify that the graveyards are empty
        for graveyard in &chess.graveyards {
            for row in &graveyard.graveyard {
                for piece in row {
                    assert_eq!(*piece, None);
                }
            }
        }
    }

    #[test]
    fn test_on_board() {
        assert!(Chess::on_board(&Pos::new(3, 3)));
        assert!(Chess::on_board(&Pos::new(0, 0)));
        assert!(!Chess::on_board(&Pos::new(-1, 2)));
        assert!(!Chess::on_board(&Pos::new(8, 5)));
    }

    #[test]
    fn test_board_positions() {
        // Get all board positions and count them
        let positions: Vec<Pos> = Chess::board_positions().collect();
        assert_eq!(positions.len(), 64);

        // Check that all positions are on the board
        for pos in &positions {
            assert!(Chess::on_board(pos));
        }
    }

    #[test]
    fn test_graveyard_positions() {
        let chess = Chess::default();
        let positions: Vec<Pos> = chess.graveyard_positions().collect();

        // Check that positions are within the expected range for the current player
        if chess.turn == Color::White {
            assert_eq!(positions.len(), 10);
            for pos in &positions {
                assert!(pos.x() >= -2 && pos.x() <= -2 && pos.y() >= -1 && pos.y() <= 8);
            }
        } else {
            assert_eq!(positions.len(), 10);
            for pos in &positions {
                assert!(pos.x() >= 9 && pos.x() <= 9 && pos.y() >= -1 && pos.y() <= 8);
            }
        }
    }

    #[test]
    fn test_border_positions() {
        // Get all border positions and count them
        let positions: Vec<Pos> = Chess::border_positions().collect();
        assert_eq!(positions.len(), 36);

        // Check that all positions are not on the board
        for pos in &positions {
            assert!(!Chess::on_board(pos));
        }
    }

    #[test]
    fn test_pieces() {
        let chess = Chess::default();
        let piece_count = chess.pieces().count();

        // Verify that the number of pieces is as expected in the initial state
        assert_eq!(piece_count, 32);
    }

    #[test]
    fn test_moves() {
        let mut chess = Chess::default();

        // Test that there are moves available in the initial state
        let initial_moves: Vec<Move> = chess.moves().collect();
        assert!(!initial_moves.is_empty());

        // Perform a move and verify that moves are still available
        let first_move = initial_moves[0];
        chess.perform(first_move);
        let updated_moves: Vec<Move> = chess.moves().collect();
        assert!(!updated_moves.is_empty());
    }

    #[test]

    fn test_is_safe() {
        let mut chess = Chess::default();

        // Create a scenario where the white king is checked
        // Perform moves (fool's mate) to put the white king in a position where it is checked
        chess.perform(Move::new(Pos::new(5, 1), Pos::new(5, 2)));
        assert!(!chess.is_checked(Color::White)); // king is still fine
        chess.perform(Move::new(Pos::new(4, 6), Pos::new(4, 5)));
        assert!(!chess.is_checked(Color::White));
        chess.perform(Move::new(Pos::new(6, 1), Pos::new(6, 3)));
        assert!(!chess.is_checked(Color::White));
        chess.perform(Move::new(Pos::new(3, 7), Pos::new(7, 3)));
        // Verify that the white king is in check
        assert!(chess.is_checked(Color::White));

        // Create a scenario where the king is not in check
        let mut safe_chess = Chess::default();
        // Perform some moves to create a safe scenario
        safe_chess.perform(Move::new(Pos::new(4, 6), Pos::new(4, 4)));

        // Verify that the king is not in check
        assert!(!safe_chess.is_checked(Color::White));
    }

    #[test]
    fn test_unsafe_moves() {
        let chess = Chess::default();

        // Test that there are unsafe moves available
        let unsafe_moves: Vec<Move> = chess.unsafe_moves(Color::White).collect();
        assert!(!unsafe_moves.is_empty());
    }

    #[test]
    fn test_perform() {
        let mut chess = Chess::default();

        // Perform a move and verify that the board state changes
        let initial_piece = chess[Pos::new(4, 6)].unwrap();
        let from = Pos::new(4, 6);
        assert_eq!(chess[from], Some(initial_piece));
        let to = Pos::new(4, 4);
        let chess_before_move = chess.clone();
        chess.perform(Move::new(from, to));

        assert_eq!(chess[to], Some(initial_piece));
        assert_eq!(chess[from], None);
        assert_eq!(chess_before_move[to], None);
    }

    #[test]
    fn test_evaluate() {
        let chess = Chess::default();
        let evaluation = chess.evaluate();
        // In the initial state, it's common to have a positive evaluation for White.
        assert!(evaluation >= 0);
    }

    #[test]
    fn test_outcome() {
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
        chess.perform(Move::new(Pos::new(3, 7), Pos::new(7, 3)));

        chess.turn = Color::White;
        // Black won by checkmating white
        assert_eq!(chess.outcome(), Some(Outcome::Winner(Color::Black)));
    }
}
