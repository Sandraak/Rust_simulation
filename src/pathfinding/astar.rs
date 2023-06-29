use crate::chess::{
    chess::{Chess},
    pos::Pos,
    BoardState,
};

pub const START_POS: Pos = Pos { x: 3, y: 2 };
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
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    path: Vec<Pos>,
    crossed_pieces: Vec<Pos>,
    capture: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Locations {
    from: Pos,
    to: Pos,
}
// struct PassedPiece {
//     piece: Vec<Locations>,
// }

pub fn calculate_path(start_pos: Pos, end_pos: Pos, boardstate: &BoardState) -> Option<Vec<Path>> {
    // Lege vector met alle paden
    let mut paths: Vec<Path> = vec![];
    // Lege vector met de origele zet en eventueel geslagen stuk
    let mut priorty_paths: Vec<Path> = vec![];

    // De originele zet
    let original_path = a_star(start_pos, end_pos, boardstate)?;
    let mut capture_path : Path = original_path.clone(); // needs to be an empty path
    // Als de originele zet en de capture geen stukken passeert, is er maar 1 pad dat de magneet moet afleggen.
    paths.push(original_path.clone());
    // Is er een stuk geslagen?
    if original_path.capture{
        capture_path = capture(end_pos, boardstate)?;
        paths.push(capture_path.clone());
    }
    
    let mut no_crossed_pieces = true;
    for path in paths.clone() {
        if !path.crossed_pieces.is_empty(){
            no_crossed_pieces = false;
        }
    }

    if no_crossed_pieces{
        return Some(paths);
    }

    // Pak de stukken die het originele pad blokkeren.
    else {
        priorty_paths = paths.clone();
        let mut obstructing_pieces: Vec<Locations> = vec![];

        for mut path in priorty_paths{

        for piece in path.crossed_pieces.clone() {
            // Vind een goede eind locatie voor het uitwijkende stuk.
            let locations = find_end_pos(piece, &paths, boardstate, &obstructing_pieces);
            // Voeg de start en eind locaties van de uitwijkende stukken toe aan de vector obstructing_pieces.
            if !obstructing_pieces.contains(&locations){
                obstructing_pieces.push(locations);
                }
            path.crossed_pieces.pop();
        }
        // Er zijn nu nieuwe posities gevonden voor alle stukken die het originele pad blokkeerden.
        // Voor deze stukken moet ook het optimale pad gevonden worden. Deze gevonden paden worden ook toegevoegd aan de vector.
        for piece in obstructing_pieces.clone() {
            let path = a_star(piece.from, piece.to, boardstate)?;
            if !paths.contains(&path) {
                paths.push(path);
            }
        }

    }

        // Het kan zijn dat een stuk moet uitwijken over een pad waar ook een stuk op staat.
        // Dit stuk moet dan ook uitwijken.
        // Controleer of er niet nog meer obstructing pieces bijkomen.
        for mut path in paths.clone() {
            if !path.crossed_pieces.is_empty() && path.path != original_path.path && path.path != capture_path.path {
                // Als er crossed pieces zijn buiten het originele pad, moeten deze ook uitwijken.
                while !path.crossed_pieces.is_empty() {
                    for piece in path.crossed_pieces.clone() {
                        let locations =
                            find_end_pos(piece, &paths, boardstate, &obstructing_pieces);
                        // Voeg de start en eind locaties van de uitwijkende stukken toe aan de vector.
                        // Verwijder het stuk uit de crossed pieces vector, zodat de loop ooit stopt.
                        if !obstructing_pieces.contains(&locations){
                        obstructing_pieces.push(locations);
                        }
                        path.crossed_pieces.pop();
                    }
                    for piece in obstructing_pieces.clone() {
                        let new_path = a_star(piece.from, piece.to, boardstate)?;
                        if !paths.contains(&new_path) {
                            paths.push(new_path);
                        }
                    }
                }
            }
        }
        // Als er voor elk van deze paden geen nieuwe obstructing pieces zijn
        // moeten de paden in paths omgedraaid worden uitgevoerd,
        // wanneer de originele zet is uitgevoerd moeten de stukken worden terug gezet.
        paths.reverse();
        let reversed = obstructing_pieces.clone();
        for piece in reversed {
            let path_back = a_star(piece.to, piece.from, boardstate)?;
            paths.push(path_back);
        }
    }
    Some(paths)
}

///Finds the shortest path on the board between a start and end position, based on the current boardstate.
fn a_star(start_pos: Pos, end_pos: Pos, boardstate: &BoardState) -> Option<Path> {
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
        // Begin bij de laatste node en kijk naar de node met de positie van parent,
        // kijk vervolgens naar zijn parent, doe dit tot de start node, dus tot parent none is.
        if current.pos == end_pos {
            println!("End reached!");
            let mut path_node = current;
            loop {
                //Check if there are any crossed pieces. The moving piece is not an obstructing piece.
                if boardstate.chess[path_node.pos].is_some()
                    && (path_node.pos != start_node.pos)
                {
                    if boardstate.chess[path_node.pos].is_some() && (path_node.pos == end_pos){
                        path.capture = true;
                    }
                    else{
                    path.crossed_pieces.push(path_node.pos);
                    }
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
    (row >= -3 && row <= 10) && (col >= -1 && col <= 8)
}

fn capture(start_pos: Pos, boardstate: &BoardState) -> Option<Path> {
    let end_pos = boardstate
        .chess
        .graveyard_positions()
        .filter(|pos| boardstate.chess[pos].is_none())
        .min_by(|a, b| a.distance(start_pos).partial_cmp(&b.distance(start_pos)).unwrap())
        .unwrap();
    a_star(start_pos, end_pos, boardstate)
}

/// Alle obstructing pieces moeten aan de kant
/// De posities van de obstructing pieces worden op geslagen in path.crossed_pieces.
/// De functie moet de obstructing pieces verplaatsen naar lege plekken op het bord (dus niet op het pad),
/// of op plekken waar andere stukken heen verplaatst zijn.
/// IDEE VOOR DE END POS: Kan ik een locatie vinden waarbij:
///       pos niet in path.path zit, nog geen ander stuk is heen verplaatst, en boardstate.chess.pos.is_empty()
///       en dan voor alle posities waarvoor dit geldt, de positie waarbij .distance het laagst is.
/// Start en eind locatie zijn nu bekend.
/// pad zoeken tussen die twee
/// Weer obstructing piece? => herhaal
/// Geen obstructing piece? => verplaats het laatst gecheckte stuk naar zijn end pos?
/// PROBLEEM?   De boardstate wordt tijdens het vinden van een pad niet geupdate
///             Hierdoor worden de oude locaties van stukken die uit de weg gaan niet als vrij gezien.
///             Deze als vrij markeren is ook geen oplossing want ze later zijn ze wellicht wel bezet  
/// Oplossingen?
///            Alles steeds opnieuw checken?
fn find_end_pos(
    start_pos: Pos,
    paths: &Vec<Path>,
    boardstate: &BoardState,
    locations: &Vec<Locations>,
) -> Locations {
    // Vind een positie die:
    // 1) niet in path.path zit
    // 2) niet in locations.to zit
    // 3) waar geen stuk staat, aka boardstate.chess[path_node.pos].is_none()
    // 4) laagste value voor .distance() van de locaties die aan bovenstaande punten voldoen.
    let end_pos = Chess::board_positions()
        .filter(|pos| {
            paths
                .iter()
                .flat_map(|path| path.path.iter())
                .find(|p| *p == pos)
                .is_none()
        }) // 1) niet in path.path
        .filter(|pos| {
            locations
                .iter()
                .map(|location| location.to)
                .find(|p| p == pos)
                .is_none()
        }) // 2) niet in locations.to
        .filter(|pos| boardstate.chess[pos].is_none()) // 3) waar geen stuk staat
        .min_by(|a, b| a.distance(start_pos).partial_cmp(&b.distance(start_pos)).unwrap()) // 4) laagste value voor .distance()
        .unwrap();
    // hier vergelijk ik de distance van a tot de start_pos en de distance van b tot de start pos.
    // cmp returnt ordering::greater als de a.distance(start_pos) groter is dan b.distance(start_pos).
    // Hij vergelijkt dit voor alle mogelijke posities.
    Locations {
        from: start_pos,
        to: end_pos,
    }
}
