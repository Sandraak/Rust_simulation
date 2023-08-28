use std::ops::Not;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{
    chess::{chess::Color, chess::Move, pos::Pos, BoardState},
    pathfinding::astar::Path,
};
use bevy::prelude::*;

static POLLING_DONE: AtomicBool = AtomicBool::new(false);
pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentPaths>()
            .init_resource::<CurrentLocations>()
            .init_resource::<MagnetStatus>()
            .init_resource::<PlayerTurn>()
            .init_resource::<Setup>()
            .insert_resource(Destination {
                goal: Pos { x: 0, y: 0 },
            })
            .insert_resource(CurrentMove {
                current_move: Move {
                    from: Pos { x: 0, y: 0 },
                    to: Pos { x: 0, y: 0 },
                },
            })
            .add_event::<MoveEvent>()
            .add_event::<NewPathEvent>()
            .add_event::<PathEvent>()
            .add_event::<FirstMoveEvent>()
            .add_event::<MagnetEvent>()
            .add_event::<EndTurnEvent>()
            .add_event::<ComputerTurnEvent>()
            .add_system(update_path)
            .add_system(update_locations)
            .add_system(update_current_pos)
            .add_system(set_first_pos)
            .add_system(poll_system)
            .add_system(end_turn);
    }
}

///Keeps track of what Color the human player is playing
/// and who's turn it is.
#[derive(Resource, Default)]
pub struct PlayerTurn {
    pub color: Color,
    pub turn: Player,
}

///The player is either a human or a computer
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum Player {
    #[default]
    Human,
    Computer,
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::Human => Player::Computer,
            Player::Computer => Player::Human,
        }
    }
}
///Keeps track of whether the setup has been performed.
#[derive(Resource, Default, Debug)]
pub struct Setup {
    pub complete: bool,
}
///Vector with all the paths the magnet still has to cover.
#[derive(Resource, Default, Debug)]
pub struct CurrentPaths {
    pub paths: Vec<Path>,
}

///Vector with all the positions of a certain path the magnet still has to cover.
#[derive(Resource, Default, Debug)]
pub struct CurrentLocations {
    pub locations: Path,
}

///The position to which the magnet is currently moving.
#[derive(Resource)]
pub struct Destination {
    pub goal: Pos,
}

///The current chess move that is being performed
#[derive(Resource)]
pub struct CurrentMove {
    pub current_move: Move,
}

/// This struct keeps track of whether the magnet is currently moving,
/// whether the magnet hsa reached its destination (simulation and real),
/// and whether the magnet is currently on.
#[derive(Resource)]
pub struct MagnetStatus {
    pub moving: bool,
    pub simulation: bool,
    pub real: bool,
    pub on: bool,
}

impl Default for MagnetStatus {
    fn default() -> Self {
        Self {
            moving: false,
            simulation: false,
            real: true,
            on: false,
        }
    }
}
pub struct MoveEvent;
pub struct NewPathEvent;
pub struct PathEvent;
pub struct FirstMoveEvent;
pub struct MagnetEvent;
pub struct EndTurnEvent;
pub struct ComputerTurnEvent;

/// System that polls to the hardware implementation whether the magnet has yet reached its destination.
/// It only polls when the magnet is moving.
/// When POLLING_DONE is true, the magnet in hardware prototype has reached its destination.
/// When this is the case, this function sends a MagnetEvent to signal that the hardware is ready for a new position.
fn poll_system(
    mut magnet_status: ResMut<MagnetStatus>,
    mut magnet_event: EventWriter<MagnetEvent>,
) {
    if magnet_status.moving {
        poll();
        if POLLING_DONE.load(Ordering::Relaxed) {
            magnet_status.real = true;
            magnet_status.moving = false;
            magnet_event.send(MagnetEvent);
        }
    }
}
/// Sends a HTTP GET request to the server running on the microcontroller.
/// It stores the response in the atomic bool POLLING_DONE.
fn poll() {
    let request = ehttp::Request::get("http://192.168.1.22/poll");
    ehttp::fetch(request, move |_result: ehttp::Result<ehttp::Response>| {
        POLLING_DONE.store(true, Ordering::Relaxed);
    });
}


/// When a new [`MoveEvent`] is registered this function sends a PathEvent which triggers the function
/// [`give_path`] in astar.rs.
/// 
/// [`give_path`]: crate::pathfinding::astar::give_path
pub(crate) fn update_path(mut new_move: EventReader<MoveEvent>, mut new_path: EventWriter<PathEvent>) {
    for _event in new_move.iter() {
        new_path.send(PathEvent);
    }
}

/// When a new [`NewPathEvent`] is registered, this function checks whether the path vector in [`CurrentPaths`]
/// is empty. If this is the case, all paths have been executed and an [`EndTurnEvent`] is send, which triggers
/// [`end_turn`].
/// When this is not the case, the first path in [`CurrentPaths`] is moved to [`CurrentLocations`], 
/// and a [`FirstMoveEvent`] is send.
fn update_locations(
    mut current_paths: ResMut<CurrentPaths>,
    mut current_locations: ResMut<CurrentLocations>,
    mut path_update: EventReader<NewPathEvent>,
    mut start_move: EventWriter<FirstMoveEvent>,
    mut end_turn: EventWriter<EndTurnEvent>,
) {
    for _event in path_update.iter() {
        if current_paths.paths.is_empty() {
            end_turn.send(EndTurnEvent);
        } else {
            current_locations.locations = current_paths.paths.first().unwrap().clone();
            current_paths.paths.remove(0);
            start_move.send(FirstMoveEvent);
        }
    }
}

/// When a new [`MagnetEvent`] is registered, this function checks whether the magnet has
/// reached its destination in both the simulation and hardware. When this is the case, the function
/// [`update_pos`] with magnet_on = True is called. 
fn update_current_pos(
    mut magnet_update: EventReader<MagnetEvent>,
    mut magnet_status: ResMut<MagnetStatus>,
    mut current_locations: ResMut<CurrentLocations>,
    mut new_pos: ResMut<Destination>,
    mut new_path: EventWriter<NewPathEvent>,
) {
    for _event in magnet_update.iter() {
        if magnet_status.simulation && magnet_status.real {
            update_pos(
                &mut magnet_status,
                &mut current_locations,
                &mut new_pos,
                &mut new_path,
                true,
            );
        }
    }
}

/// When a new [`FirstMoveEvent`] is registered, this function checks whether the magnet has
/// reached its destination in both the simulation and hardware. When this is the case, the function
/// [`update_pos`] with magnet_on = False is called. The magnet must be off during this move because the 
/// first position of a path is never part of the intended move, but puts the magnet in place for said move.
fn set_first_pos(
    mut first_move: EventReader<FirstMoveEvent>,
    mut magnet_status: ResMut<MagnetStatus>,
    mut current_locations: ResMut<CurrentLocations>,
    mut new_pos: ResMut<Destination>,
    mut new_path: EventWriter<NewPathEvent>,
) {
    for _event in first_move.iter() {
        update_pos(
            &mut magnet_status,
            &mut current_locations,
            &mut new_pos,
            &mut new_path,
            false,
        );
    }
}


/// Updates the next position of the magnet based on the value in [`CurrentLocations`]. When this
/// vector is empty, there are no moves in the current path and a [`NewPathEvent`] is called, which triggers
/// [`update_locations`]. When there are still locations, the first Pos of the path vector
/// in  [`CurrentLocations`] is moved to [`Destination`]. The magnet will move to this position,
/// so neither the simulation or real magnet has yet reached it,
/// putting these values to false and magnet_moving to true.
/// The parameter magnet_on determines whether the magnet is on or off during this move. A HTTP request
/// containing the position in [`Destination`] and the magnet_on Bool is send to the hardware.
fn update_pos(
    magnet_status: &mut ResMut<MagnetStatus>,
    current_locations: &mut ResMut<CurrentLocations>,
    new_pos: &mut ResMut<Destination>,
    new_path: &mut EventWriter<NewPathEvent>,
    magnet_on: bool,
) {
    if current_locations.locations.positions.is_empty() {
        new_path.send(NewPathEvent);
    } else {
        let goal = *current_locations.locations.positions.first().unwrap();
        **new_pos = Destination { goal };
        current_locations.locations.positions.remove(0);
        magnet_status.simulation = false;
        magnet_status.real = false;
        magnet_status.on = magnet_on;

        let goal_url = format!(
            "{}/{}/{}/{}",
            "http://192.168.1.22",
            goal.x(),
            goal.y(),
            magnet_status.on as isize
        );
        let request = ehttp::Request::get(goal_url);
        ehttp::fetch(request, move |_result: ehttp::Result<ehttp::Response>| {});
        magnet_status.moving = true;
    }
}

/// When a new [`EndTurnEvent`] is registered this function checks whether this function has been called before.
/// When this is not the case, it sets the value in [`Setup`] to true. If this function is called when setp is true,
/// this means the move has been executed. The function updates all the resources linked to the current turn.
/// After this, the system is ready for a new move from either computer or human player.
fn end_turn(
    mut end_turn: EventReader<EndTurnEvent>,
    mut computer_turn: EventWriter<ComputerTurnEvent>,
    mut current_locations: ResMut<CurrentPaths>,
    mut magnet_status: ResMut<MagnetStatus>,
    mut player_turn: ResMut<PlayerTurn>,
    mut boardstate: ResMut<BoardState>,
    current_move: Res<CurrentMove>,
    mut setup: ResMut<Setup>,
) {
    for _event in end_turn.iter() {
        if setup.complete {
            *current_locations = CurrentPaths { paths: vec![] };
            magnet_status.on = false;
            magnet_status.moving = false;
            let m = current_move.current_move;
            boardstate.chess.perform(m);
            player_turn.turn = !player_turn.turn;
            if player_turn.turn == Player::Computer {
                computer_turn.send(ComputerTurnEvent);
            }
        } else {
            setup.complete = true;
        }
    }
}
