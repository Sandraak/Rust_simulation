use crate::{
    chess::{
        chess::{Chess, Move},
        pos::Pos,
        BoardState,
    },
    controller::controller::{CurrentMove, CurrentPaths, NewPathEvent, PathEvent},
};
use bevy::prelude::{App, EventReader, EventWriter, Plugin, Res, ResMut};

/// Node used for the A* algorithm
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
struct Node {
    pos: Pos,
    distance_to_start: u8,
    distance_to_end: u8,
    parent: Option<Pos>,
}

impl Node {
    fn total_cost(&self) -> u8 {
        self.distance_to_start + self.distance_to_end
    }
}
///All the paths and crossed pieces in separate vectors
#[derive(Debug, Clone, PartialEq, Default)]
struct PathInformation {
    path: Path,
    crossed_pieces: Vec<Pos>,
    capture: bool,
}
///Vector of positions
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub positions: Vec<Pos>,
}

impl Default for Path {
    fn default() -> Self {
        Self {
            positions: Default::default(),
        }
    }
}

impl IntoIterator for Path {
    type Item = Pos;
    type IntoIter = <Vec<Pos> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.positions.into_iter()
    }
}

/// Plugin for running the systems needed by the bevy app.
pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(give_path);
    }
}
/// When a new [`Pathevent`] is registerd, this function will update [`CurrentPaths`] to
/// a vector of paths returned by [`calculate_path`]. It then sends a [`NewPathEvent`] which
/// triggers [`update_locations`] in controller.rs
/// 
/// [`Pathevent`]: crate::controller::controller::PathEvent
/// [`update_locations`]: crate::controller::controller::update_locations
pub (crate) fn give_path(
    mut new_move: EventReader<PathEvent>,
    current_move: Res<CurrentMove>,
    boardstate: Res<BoardState>,
    mut current_locations: ResMut<CurrentPaths>,
    mut new_locations: EventWriter<NewPathEvent>,
) {
    for _event in new_move.iter() {
        *current_locations = CurrentPaths {
            paths: calculate_path(&current_move, &boardstate).unwrap(),
        };
        new_locations.send(NewPathEvent);
    }
}


/// Calculates all the paths that are necessary for a move to occur without any collisions.
/// Any obstructing pieces will first move out of the way of a captured piece that is moving to the graveyard.
/// After this piece has reached the graveyard, any obstructing pieces will move out of the way of the attacking piece,
/// When the attacking piece has reached its destination. The the pieces that moved out of the attacking piece's way 
/// will return to their original positions. Then pieces that moved out of the captured piece's way will return
/// to their original positions. 
fn calculate_path(mov: &Res<CurrentMove>, boardstate: &Res<BoardState>) -> Option<Vec<Path>> {
    let mut paths_info: Vec<PathInformation> = vec![];
    // The path for the original move as received by the controller.
    let original_path_info = a_star(mov.current_move.from, mov.current_move.to, boardstate)?;
    // Information about the path for the (optional) captured piece. Should be changed to a default value of Pathinformation
    let mut capture_path_info: PathInformation = original_path_info.clone();                                                                           
    paths_info.push(original_path_info.clone());
    // If a piece has been captured, calculate a path to the graveyard for this piece.
    if original_path_info.capture {
        capture_path_info = capture(mov.current_move.to, boardstate)?;
        // If no pieces have been crossed in the original path,
        // the captured piece should move first and thus be the first element
        // in the paths vector.
        if original_path_info.crossed_pieces.is_empty() {
            paths_info.insert(0, capture_path_info.clone());
        } else {
            // The vector will be reversed when pieces have been crossed, so for the 
            // captured piece to move first its path has to be put and the back of the vector
            paths_info.push(capture_path_info.clone());
        }
    }
    // checks whether no pieces are crossed in either the original path 
    // or the captured piece's path.
    let mut no_crossed_pieces = true;
    for path_info in paths_info.clone() {
        if !path_info.crossed_pieces.is_empty() {
            no_crossed_pieces = false;
        }
    }
    // If no pieces have been crossed,the paths will be returned.
    if no_crossed_pieces {
        return Some(
            paths_info
                .into_iter()
                .map(|path_info| path_info.path)
                .collect(),
        );
    }
    // If pieces have been crossed, first the pieces obstructing the path of the original move will be checked
    else {
        // The priority paths are the paths of the  original move and the captured piece
        let priority_paths_info = paths_info.clone();
        let mut obstructing_pieces: Vec<Move> = vec![];
        for mut path_info in priority_paths_info {
            for piece in path_info.crossed_pieces.clone() {
                // Finds a location for the obstructing pieces
                let locations = find_end_pos(piece, &paths_info, boardstate, &obstructing_pieces);
                // Adds the start and end locations to obstructing_pieces
                if !obstructing_pieces.contains(&locations) {
                    obstructing_pieces.push(locations);
                }
                // the piece is moved out of the way and is no longer obstructing the path.
                // thus it can be removed from crossed_pieces.
                path_info.crossed_pieces.pop();
            }
            // new locations have been found for the obstructing pieces, now a path needs to be found.
            // Add these paths, and their information to the paths_info vector.
            for piece in obstructing_pieces.clone() {
                let path_info = a_star(piece.from, piece.to, boardstate)?;
                if !paths_info.contains(&path_info) {
                    paths_info.push(path_info);
                }
            }
        }
        // When the path of an obstructing piece moving out of the way also crosses a piece, 
        // new paths need to be found.
        for mut path_info in paths_info.clone() {
            if !path_info.crossed_pieces.is_empty()
                && path_info.path != original_path_info.path
                && path_info.path != capture_path_info.path
            {
                // This code will run for as long as the path of an obstructing piece moving out of the way results
                // in another obstructing piece.
                while !path_info.crossed_pieces.is_empty() {
                    for piece in path_info.crossed_pieces.clone() {
                        let locations =
                            find_end_pos(piece, &paths_info, boardstate, &obstructing_pieces);
                        // Adds the start and end locations to obstructing_pieces
                        if !obstructing_pieces.contains(&locations) {
                            obstructing_pieces.push(locations);
                        }
                        // the piece will be moved out of the way and is no longer obstructing the path.
                        // thus it can be removed from crossed_pieces.
                        path_info.crossed_pieces.pop();
                    }
                    // Find a new path for all the obstructing pieces that have no path yet.
                    for piece in obstructing_pieces.clone() {
                        let new_path = a_star(piece.from, piece.to, boardstate)?;
                        if !paths_info.contains(&new_path) {
                            paths_info.push(new_path);
                        }
                    }
                }
            }
        }

        // Puts all the paths in the correct order, and makes sure the obstructing pieces
        // move back to their original locations after moving out of the way.
        paths_info.reverse();
        let mut paths_info_normal = paths_info.clone();
        paths_info_normal.reverse();
        for path in paths_info_normal.clone() {
            let mut path_back = PathInformation::default();
            let compare_path = path.clone();
            path_back.path.positions = path.path.positions;
            path_back.path.positions.reverse();

            if original_path_info.path != compare_path.path
                && capture_path_info.path != compare_path.path
            {
                paths_info.push(path_back);
            }
        }
    }
    //Return all the paths
    Some(
        paths_info
            .into_iter()
            .map(|path_info| path_info.path)
            .collect(),
    )
}

/// Finds the shortest path using the a* algorithm on the board between a start and end position,
/// based on the current boardstate.
/// The function returns a path, and information about captured and crossed pieces on that path.
fn a_star(start_pos: Pos, end_pos: Pos, boardstate: &Res<BoardState>) -> Option<PathInformation> {
    let start_node: Node = Node {
        pos: start_pos,
        distance_to_start: 0,
        distance_to_end: start_pos.distance(end_pos) as u8,
        parent: None,
    };
    if !within_bounds(start_pos.x, start_pos.y) || !within_bounds(end_pos.x, end_pos.y) {
        return None;
    }

    let mut open_list: Vec<Node> = vec![];
    let mut closed_list: Vec<Node> = vec![];
    let mut path_info = PathInformation {
        path: Path { positions: vec![] },
        crossed_pieces: vec![],
        capture: false,
    };
    open_list.push(start_node);

    loop {
        //The current node is the one with the shortest total cost in the open list
        let current = *open_list
            .iter()
            .min_by(|a, b| a.total_cost().cmp(&b.total_cost()))?;
        closed_list.push(current);
        open_list.retain(|node| *node != current);
        // Start at the last node and look at the node with the position of the Parent.
        // Follow by looking at the parent's parent, repeat until the start_node, which has no parent.
        if current.pos == end_pos {
            let mut path_node = current;
            loop {
                //Check if there are any crossed pieces. The moving piece is not an obstructing piece.
                if boardstate.chess[path_node.pos].is_some() && (path_node.pos != start_node.pos) {
                    if boardstate.chess[path_node.pos].is_some() && (path_node.pos == end_pos) {
                        path_info.capture = true;
                    } else {
                        path_info.crossed_pieces.push(path_node.pos);
                    }
                }
                path_info.path.positions.push(path_node.pos);
                //Only the start node doesn't have a parent, so when the node has no parent, we're back at the start.
                if path_node.parent.is_none() {
                    path_info.path.positions.reverse();
                    return Some(path_info);
                }
                //Find the parent node's position.
                path_node = *closed_list
                    .iter()
                    .find(|node| path_node.parent.unwrap() == node.pos)
                    .unwrap();
            }
        }
        //loop through neighbours
        for row in -1..=1 {
            for col in -1..=1 {
                //check if it's not itself and within the moveable space.
                if !((row == 0 && col == 0) && within_bounds(row, col)) {
                    let pos = Pos {
                        x: current.pos.x + row,
                        y: current.pos.y + col,
                    };
                    let mut cost = 4;
                    //Check diagonal
                    if row != 0 && col != 0 {
                        cost += 1;
                    }
                    // Check whether there is a piece
                    // and update the cost for passing through
                    if boardstate.chess[pos].is_some() {
                        cost += 12;
                    }
                    let distance_to_start: u8 = current.distance_to_start + cost; // add the cost for the movement to the distance
                    let distance = pos.distance(end_pos);
                    let neighbor: Node = Node {
                        pos,
                        distance_to_start,
                        distance_to_end: distance as u8,
                        parent: Some(current.pos),
                    };
                    // When the neighbor is not in the closed list check the open list.
                    if closed_list.iter().find(|n| neighbor.pos == n.pos).is_none() {
                        //Check whether the neighbour is already in the open list,
                        // if so check if the distance to the startnode is smaller than the previous distance to the start node.
                        // when this is true, update the parent node to the current node and update the distance to the start_node.
                        match open_list.iter_mut().find(|n| neighbor.pos == n.pos) {
                            Some(old) => {
                                if neighbor.distance_to_start < old.distance_to_start {
                                    old.parent = Some(current.pos);
                                    old.distance_to_start = distance_to_start;
                                }
                            }
                            //When the neigbor is not already in the open list, add it to the open list.
                            None => {
                                open_list.push(neighbor);
                            }
                        }
                    }
                }
            }
        }
        //When the open list is empty, there is no path.
        if open_list.is_empty() {
            return None;
        }
    }
}

/// Checks whether a given position is on the board.
fn within_bounds(row: isize, col: isize) -> bool {
    (row >= -3 && row <= 10) && (col >= -1 && col <= 8)
}

///Finds a path to the graveyard for a captured piece.
fn capture(start_pos: Pos, boardstate: &Res<BoardState>) -> Option<PathInformation> {
    let end_pos = boardstate
        .chess
        .graveyard_positions()
        .filter(|pos| boardstate.chess[pos].is_none())
        .min_by(|a, b| {
            a.distance(start_pos)
                .partial_cmp(&b.distance(start_pos))
                .unwrap()
        })
        .unwrap();
    a_star(start_pos, end_pos, boardstate)
}


/// Finds a position for a obstructing piece to move to such that the position: 
/// 1) is not on the path
/// 2) is not occupied by another obstructing piece
/// 3) not occupied by another piece
/// 4) is closest to the start position of the obstructing piece
fn find_end_pos(
    start_pos: Pos,
    paths: &Vec<PathInformation>,
    boardstate: &BoardState,
    locations: &Vec<Move>,
) -> Move {
    let end_pos = Chess::board_positions()
        .filter(|pos| {
            paths
                .iter()
                .flat_map(|path_info| path_info.path.positions.iter())
                .find(|p| *p == pos)
                .is_none()
        }) // 1) not in path_info.path
        .filter(|pos| {
            locations
                .iter()
                .map(|location| location.to)
                .find(|p| p == pos)
                .is_none()
        }) // 2) not in locations.to
        .filter(|pos| boardstate.chess[pos].is_none()) // 3) waar geen stuk staat
        .chain(Chess::border_positions())
        .min_by(|a, b| {
            a.distance(start_pos)
                .partial_cmp(&b.distance(start_pos))
                .unwrap()
        }) // 4) lowest value for .distance()
        .unwrap();
    Move {
        from: start_pos,
        to: end_pos,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::{BoardState,chess::{Piece, Color}};

    #[test]
    fn test_within_bounds() {
        assert!(within_bounds(0, 0));
        assert!(within_bounds(5, 5));
        assert!(within_bounds(-3, 0));
        assert!(within_bounds(10, 8));
        assert!(!within_bounds(-4, 0));
        assert!(!within_bounds(11, 5));
    }
    #[test]
    fn test_find_end_pos() {
        // Create a board state with some pieces
        let mut board_state = BoardState::default();
        board_state.chess.board[2][2] =  Some(Piece::WHITE_ROOK);
        board_state.chess.board[3][2] = Some(Piece::BLACK_PAWN);
        board_state.chess.board[2][3] = Some(Piece::BLACK_KNIGHT);
        board_state.chess.turn = Color::White;

        // Define the start position, paths, and occupied locations
        let start_pos = Pos::new(2, 2);
        let paths_info = vec![
            PathInformation {
                path: Path { positions: vec![Pos::new(2, 2), Pos::new(3, 2), Pos::new(4, 2)] },
                crossed_pieces: vec![],
                capture: false,
            },
            PathInformation {
                path: Path { positions: vec![Pos::new(2, 2), Pos::new(2, 3), Pos::new(2, 4)] },
                crossed_pieces: vec![],
                capture: false,
            },
        ];
        let locations = vec![
            Move { from: Pos::new(2, 2), to: Pos::new(3, 2) },
            Move { from: Pos::new(2, 2), to: Pos::new(2, 3) },
        ];

        // Call find_end_pos function
        let end_pos = find_end_pos(start_pos, &paths_info, &board_state, &locations);

        // Assert that the end position meets the criteria
        assert_eq!(end_pos.from, start_pos);
        assert_ne!(end_pos.to, start_pos);
        assert!(!paths_info.iter().any(|info| info.path.positions.contains(&end_pos.to)));
        assert!(!locations.iter().any(|loc| loc.to == end_pos.to));
        assert!(board_state.chess[end_pos.to].is_none());
    }
}