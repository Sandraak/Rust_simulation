use crate::chess::{pos::Pos, BoardState};

pub const START_NODE :Node = Node{
    pos: Pos { x: 3, y: 3 },
    distance_to_start: 0,
    distance_to_end: 4,
    parent: None,
};

pub const END_NODE :Node = Node{
    pos: Pos { x: 5, y: 5},
    distance_to_start: 4,
    distance_to_end: 0,
    parent: None,
};


#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
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

pub fn a_star(start_node: Node, end_node: Node, boardstate: BoardState) -> Option<Path> {
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
        open_list.retain(|node| *node == current);
        closed_list.push(current);

        // Begin bij de laatste node en kijk naar de node met de positie van parent,
        // kijk vervolgens naar zijn parent, doe dit tot de start node, dus tot parent none is.
        if current == end_node {
            let mut path_node =current;
            loop {
                path.path.push(path_node.pos);
                if path_node.parent.is_none(){
                    path.path.reverse();
                    return Some(path);
                }
                //Vind de positie van de parent "node".
                path_node = *closed_list.iter().find(|node| path_node.parent.unwrap() == node.pos).unwrap(); 
            }
        }

        if open_list.is_empty(){
            return None;
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
                    if boardstate.chess[pos].is_some() {
                        cost = 5;
                        path.crossed_pieces.push(pos);
                    }
                    let distance_to_start: u8 = current.distance_to_start + cost; // schuin is even snel als rechtdoor
                    let distance = pos.distance(end_node.pos);
                    let mut neighbor: Node = Node {
                        pos: pos,
                        distance_to_start: distance_to_start,
                        distance_to_end: distance as u8,
                        parent: Some(current.pos),
                    };
                    // When the neighbor is not in the closed list check the open list.
                    if closed_list
                        .iter()
                        .find(|neighbor| neighbor.pos == current.pos)
                        .is_none()
                    {
                        //Check whether the neighbour is already in the open list,
                        // if so check if the distance to the startnode is smaller than the previous distance to the start node.
                        // when this is true, update the parent node to the current node and update the distance to the start_node.
                        match open_list.iter_mut().find(|n| neighbor.pos == n.pos) {
                            Some(old) => {
                                if neighbor.distance_to_start < old.distance_to_start {
                                    neighbor.parent = Some(current.pos);
                                    neighbor.distance_to_start = distance_to_start;
                                }
                            }
                            //When the neigbor is not already in the open list, add it to the open list.
                            None => {
                                neighbor.parent = Some(current.pos);
                                neighbor.distance_to_start = distance_to_start;
                                neighbor.distance_to_end =
                                    neighbor.pos.distance(end_node.pos) as u8;
                                open_list.push(neighbor);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn move_obstructing_pieces(path : Path, boardstate : BoardState) -> Path{
    let path :Path =  Path {
        path: vec![],
        crossed_pieces: vec![],
    };
    path
}

