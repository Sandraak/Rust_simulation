use std::thread::sleep;

use crate::chess::{pos::Pos, BoardState};

// pub const START_NODE: Node = Node {
//     pos: Pos { x: 3, y: 3 },
//     distance_to_start: 0,
//     distance_to_end: 2,
//     parent: None,
// };

// pub const END_NODE: Node = Node {
//     pos: Pos { x: 5, y: 5 },
//     distance_to_start: 2,
//     distance_to_end: 0,
//     parent: None,
// };

pub const START_POS: Pos = Pos { x: 2, y: 2 };

pub const END_POS: Pos = Pos { x: 7, y: 7 };

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct Node {
    pos: Pos,
    distance_to_start: u8,
    distance_to_end: u8,
    parent: Option<Pos>, //Parent is geen Node want recursieve memory meuk,
}

impl Node {
    fn total_cost(&self) -> u8 {
        self.distance_to_start + self.distance_to_end
    }
}
#[derive(Debug)]
pub struct Path {
    path: Vec<Pos>,
    crossed_pieces: Vec<Pos>,
}

pub fn a_star(start_pos: Pos, end_pos: Pos, boardstate: BoardState) -> Option<Path> {
    let start_node: Node = Node {
        pos: start_pos,
        distance_to_start: 0,
        distance_to_end: start_pos.distance(end_pos) as u8,
        parent: None,
    };
    let mut open_list: Vec<Node> = vec![];
    let mut closed_list: Vec<Node> = vec![];
    let mut path = Path {
        path: vec![],
        crossed_pieces: vec![],
    };
    open_list.push(start_node);
    // print!("closed_list: {:?}", closed_list);
    // print!("open_list: {:?}", open_list);

    loop {
        // print!("inside the loop");
        //The current node is the one with the shortest total cost in the open list
        let current = *open_list
            .iter()
            .min_by(|a, b| a.total_cost().cmp(&b.total_cost()))?;
        closed_list.push(current);
        open_list.retain(|node| *node != current);

        // println!("open_list2: {:?}", open_list );


        // print!("open_list 3: {:?}", open_list);

        
        // Begin bij de laatste node en kijk naar de node met de positie van parent,
        // kijk vervolgens naar zijn parent, doe dit tot de start node, dus tot parent none is.
        if current.pos == end_pos {
            print!("End reached!");
            let mut path_node = current;
            loop {
                path.path.push(path_node.pos);
                if path_node.parent.is_none() {
                    path.path.reverse();
                println!("SUCCES!!!");
                    return Some(path);
                }
                //Vind de positie van de parent "node".
                path_node = *closed_list
                .iter()
                .find(|node| path_node.parent.unwrap() == node.pos)
                .unwrap();
            }
        }
        //loop through neighbours
        for row in -1..=1 {
            for col in -1..=1 {
                //if check if it's not itself
                if !(row == 0 && col == 0) {
                    let pos = Pos {
                        x: current.pos.x + row,
                        y: current.pos.y + col,
                    };
                    // Check whether there is a piece
                    let mut cost = 1;
                    // Check if the piece has already been added
                    if boardstate.chess[pos].is_some() {
                        cost = 5;
                        // if path.crossed_pieces.contains(&pos) {
                        //     path.crossed_pieces.push(pos);
                        // }
                    }
                    let distance_to_start: u8 = current.distance_to_start + cost; // schuin is even snel als rechtdoor
                    let distance = pos.distance(end_pos);
                    let neighbor: Node = Node {
                        pos: pos,
                        distance_to_start: distance_to_start,
                        distance_to_end: distance as u8,
                        parent: Some(current.pos),
                    };
                    // println!("neighbour: {:?}", neighbor.pos);

                    // println!("neigbor of: {:?} is: {:?}", current.pos, neighbor.pos);
                    // When the neighbor is not in the closed list check the open list.
                    if closed_list
                        .iter()
                        .find(|n| neighbor.pos == n.pos)
                        .is_none()
                    {
                        // print!("neigbor not in closed list");
                        //Check whether the neighbour is already in the open list,
                        // if so check if the distance to the startnode is smaller than the previous distance to the start node.
                        // when this is true, update the parent node to the current node and update the distance to the start_node.
                        match open_list.iter_mut().find(|n| neighbor.pos == n.pos) {
                            Some(old) => {
                                if neighbor.distance_to_start < old.distance_to_start {
                                    old.parent = Some(current.pos);
                                    old.distance_to_start = distance_to_start;
                                }
                                // print!(
                                //     "node updated, old distance to start {:?}, new : {:?}",
                                //     old.distance_to_start, neighbor.distance_to_start
                                // );
                            }
                            //When the neigbor is not already in the open list, add it to the open list.
                            None => {
                                // neighbor.parent = Some(current.pos);
                                // neighbor.distance_to_start = distance_to_start;
                                // neighbor.distance_to_end =
                                //     neighbor.pos.distance(end_pos) as u8;
                                open_list.push(neighbor);
                                // print!("openlist: {:?}", open_list);
                            }
                        }
                    }
                }
            }
        }
        if open_list.is_empty() {
            println!("OPEN LIST EMPTYY");
            return None;
        }
    }
}

fn move_obstructing_pieces(path: Path, boardstate: BoardState) -> Path {
    let path: Path = Path {
        path: vec![],
        crossed_pieces: vec![],
    };
    path
}
