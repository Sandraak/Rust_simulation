use crate::chess::{pos::Pos, BoardState};

pub const START_POS: Pos = Pos { x: 3, y: 0 };
pub const END_POS: Pos = Pos { x: 3, y: 7 };

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct Node {
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
///All the positions and crossed pieces in separate vectors
#[derive(Debug)]
pub struct Path {
    path: Vec<Pos>,
    crossed_pieces: Vec<Pos>,
}

pub struct Capture {
    captured_piece: Pos,
}

///Finds the shortest path on the board between a start and end position, based on the current boardstate.
pub fn a_star(start_pos: Pos, end_pos: Pos, boardstate: BoardState) -> Option<Path> {
    let start_node: Node = Node {
        pos: start_pos,
        distance_to_start: 0,
        distance_to_end: start_pos.distance(end_pos) as u8,
        parent: None,
    };

    if !within_bounds(start_pos.x, start_pos.y) || !within_bounds(end_pos.x, end_pos.y) {
        println!("start or end position not on the board");
        return None;
    }

    let mut open_list: Vec<Node> = vec![];
    let mut closed_list: Vec<Node> = vec![];
    let mut path = Path {
        path: vec![],
        crossed_pieces: vec![],
    };
    open_list.push(start_node);

    loop {
        //The current node is the one with the shortest total cost in the open list
        let current = *open_list
            .iter()
            .min_by(|a, b| a.total_cost().cmp(&b.total_cost()))?;
        closed_list.push(current);
        open_list.retain(|node| *node != current);
        // Begin bij de laatste node en kijk naar de node met de positie van parent,
        // kijk vervolgens naar zijn parent, doe dit tot de start node, dus tot parent none is.
        if current.pos == end_pos {
            print!("End reached!");
            let mut path_node = current;
            loop {
                //Check if there are any crossed pieces.
                if boardstate.chess[path_node.pos].is_some() && path_node.pos != start_node.pos {
                    path.crossed_pieces.push(path_node.pos);
                }
                path.path.push(path_node.pos);
                //Only the start node doesn't have a parent, so when the node has no parent, we're back at the start.
                if path_node.parent.is_none() {
                    path.path.reverse();
                    return Some(path);
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
                    let mut cost = 10;
                    //Check diagonal
                    if row != 0 && col != 0 {
                        cost += 5;
                    }
                    // Check whether there is a piece
                    // and update the cost for passing through
                    if boardstate.chess[pos].is_some() {
                        cost += 40;
                    }
                    let distance_to_start: u8 = current.distance_to_start + cost; // schuin is even snel als rechtdoor
                    let distance = pos.distance(end_pos);
                    let neighbor: Node = Node {
                        pos: pos,
                        distance_to_start: distance_to_start,
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

fn within_bounds(row: isize, col: isize) -> bool {
    (row >= -1 && row <= 8) && (col >= -1 && col <= 8)
}

fn move_obstructing_pieces(path: Path, boardstate: BoardState) -> Path {
    let path: Path = Path {
        path: vec![],
        crossed_pieces: vec![],
    };
    path
}
