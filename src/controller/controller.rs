use std::ops::Not;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{
    chess::{chess::Color, chess::Move, pos::Pos, BoardState},
    pathfinding::astar::Path,
};
use bevy::prelude::*;

static POLLING_DONE: AtomicBool = AtomicBool::new(false);

/// env var URL: export URL=http://192.168.1.22
pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentPaths>()
            .init_resource::<CurrentLocations>()
            .init_resource::<MagnetStatus>()
            .init_resource::<PlayerTurn>()
            .init_resource::<Setup>()
            // .init_resource::<PollFutureResource>()
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

#[derive(Resource, Default)]
pub struct PlayerTurn {
    pub color: Color,
    pub turn: Player,
}

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

#[derive(Resource, Default, Debug)]
pub struct Setup {
    pub complete: bool,
}

#[derive(Resource, Default, Debug)]
pub struct CurrentPaths {
    pub paths: Vec<Path>,
}
#[derive(Resource, Default, Debug)]
pub struct CurrentLocations {
    pub locations: Path,
}

#[derive(Resource)]
pub struct Destination {
    pub goal: Pos,
}

#[derive(Resource)]
pub struct CurrentMove {
    pub current_move: Move,
}

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
            real: true, //needs setup
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

fn poll() {
    let request = ehttp::Request::get("http://192.168.1.22/poll");
    ehttp::fetch(request, move |_result: ehttp::Result<ehttp::Response>| {
        POLLING_DONE.store(true, Ordering::Relaxed);
    });
}

/// Wanneer de CurrentMove resource verandert, stuurt de chess computer een MoveEvent dat dit is gebeurd.
/// update_path reageert op dit event door een PathEvent te sturen
/// De functie give_path in het Pathfinding component luistert naar dit event en update de CurrentLocations resource.
fn update_path(mut new_move: EventReader<MoveEvent>, mut new_path: EventWriter<PathEvent>) {
    for _event in new_move.iter() {
        new_path.send(PathEvent);
        println!("PathEvent send");
    }
}

fn update_locations(
    mut current_paths: ResMut<CurrentPaths>,
    mut current_locations: ResMut<CurrentLocations>,
    mut path_update: EventReader<NewPathEvent>,
    mut start_move: EventWriter<FirstMoveEvent>,
    mut end_turn: EventWriter<EndTurnEvent>,
) {
    for _event in path_update.iter() {
        println!("update locations");
        if current_paths.paths.is_empty() {
            println!("end turn event");
            end_turn.send(EndTurnEvent);
        } else {
            current_locations.locations = current_paths.paths.first().unwrap().clone();
            current_paths.paths.remove(0);
            start_move.send(FirstMoveEvent);
            println!("FirstMove Event Send!");
        }
    }
}

/// wanneer de locatie bereikt is,
/// Update de current locatie naar devolgende in de lijst locaties.
/// Zorg er voor dat de magneten gaan veranderen wanneer deze locatie verandert.
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

fn set_first_pos(
    mut first_move: EventReader<FirstMoveEvent>,
    mut magnet_status: ResMut<MagnetStatus>,
    mut current_locations: ResMut<CurrentLocations>,
    mut new_pos: ResMut<Destination>,
    mut new_path: EventWriter<NewPathEvent>,
    // mut player_turn: ResMut<PlayerTurn>,
) {
    for _event in first_move.iter() {
        println!("set_first_pos");
        update_pos(
            &mut magnet_status,
            &mut current_locations,
            &mut new_pos,
            &mut new_path,
            false,
        );
        // player_turn.turn = true;
    }
}

fn update_pos(
    magnet_status: &mut ResMut<MagnetStatus>,
    current_locations: &mut ResMut<CurrentLocations>,
    new_pos: &mut ResMut<Destination>,
    new_path: &mut EventWriter<NewPathEvent>,
    magnet_on: bool,
) {
    if current_locations.locations.positions.is_empty() {
        println!("new path event");
        new_path.send(NewPathEvent);
        println!("new path event send");
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
// }

/// activate when all the locations have been reached.
/// reset all the resources linked to the current turn.
/// update the boardstate
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
            // magnet_status.simulation = false;
            // magnet_status.real = false;
            magnet_status.on = false;
            magnet_status.moving = false;
            // player_turn.turn = false;
            let m = current_move.current_move;
            boardstate.chess.perform(m);
            player_turn.turn = !player_turn.turn;
            if player_turn.turn == Player::Computer {
                computer_turn.send(ComputerTurnEvent);
                println!("computer turn event send");
            }
        } else {
            println!("SETUP COMPLETE");
            setup.complete = true;
        }
    }
}

// / TODO:
// / 1) Maak een systeem dat een nieuwe pad berekent wanneer de current move verandert.
// /     a) Stuur deze Move naar het pathfinding component
// /     b) Zorgt er voor dat pathfinding een pad gaat berekenen wanneer hij een nieuwe move krijgt.
// / 3) Hak dit pad op in locaties
// /     a) stuur deze locaties 1 voor 1 naar zowel hardware als simulatie
// /     b) wanneer de laatste locatie bereikt is, reset alles.
// /
// / Maak een systeem dat de volgende positie naar de simulatie en hardware stuurt.
// / Hij moet dit doen zodra de magneet op beide plekken op de juiste positie is.
// / Meegeven of de magneet uit of aan is.
// /
// / In magnet.rs zitten 2 (3?) systemen die hier interactie mee hebben.
// / 1) Systeem dat reageert wanneer het een nieuwe positie krijgt en deze update naar de ontvangen waarde.
// / 2) Systeem dat wanneer de magneet niet op deze positie is, naar deze positie beweegt.
// / 3) Systeem dat een bericht naar de controller stuurt wanneer de positie bereikt is.
// /
// / De hardware bevat dezelfde systemen.
