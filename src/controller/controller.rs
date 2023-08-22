use std::{env, ops::Not};
use std::future::Future;

use crate::{
    chess::{chess::Color, chess::Move, pos::Pos, BoardState},
    pathfinding::astar::Path,
};
use bevy::prelude::*;
use std::str::FromStr;
use std::task::Poll;
use reqwest::Response;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentPaths>()
            .init_resource::<CurrentLocations>()
            .init_resource::<MagnetStatus>()
            .init_resource::<PlayerTurn>()
            .init_resource::<Setup>()
            .init_resource::<PollFutureResource>()
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
            .add_system(poll)
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
    pub simulation: bool,
    pub real: bool,
    pub on: bool,
}

impl Default for MagnetStatus {
    fn default() -> Self {
        Self {
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

#[derive(Default, Resource)]
struct PollFutureResource(PollFutureState);

#[derive(Default)]
enum PollFutureState {
    NotStarted,
    AwaitGet(Box<dyn Future<Output = reqwest::Result<Response>>>),
    AwaitText(Box<dyn Future<Output = reqwest::Result<String>>>),
}

fn poll(
    mut poll_state: ResMut<PollFutureState>,
    mut magnet_status: ResMut<MagnetStatus>,
    mut magnet_update: EventWriter<MagnetEvent>,
) {
    // Manually "await" futures
    match &*poll_state {
        PollFutureState::NotStarted => {
            let poll_url = format!("{}/poll", env::var("URL").expect("URL not set"));
            let future = reqwest::get(&poll_url);
            *poll_state = PollFutureState::AwaitGet(Box::new(future));
        }
        PollFutureState::AwaitGet(future) => {
            match future.poll() {
                Poll::Ready(output) => {
                    let future = output.unwrap().text();
                    *poll_state = PollFutureState::AwaitText(Box::new(future))
                }
                Poll::Pending => {
                    // Wait for the next call of poll
                }
            }
        }
        PollFutureState::AwaitText(future) => {
            match future.poll() {
                Poll::Ready(output) => {
                    let resp = output.unwrap();
                    if bool::from_str(&resp).unwrap() {
                        magnet_status.real = true;
                        magnet_update.send(MagnetEvent);
                    };
                    *poll_state = PollFutureState::NotStarted;
                }
                Poll::Pending => {
                    // Wait for the next call of poll
                }
            }
        }
    }
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
    } else {
        let goal = *current_locations.locations.positions.first().unwrap();
        **new_pos = Destination { goal };
        current_locations.locations.positions.remove(0);
        magnet_status.simulation = false;
        // magnet_status.real = false;
        magnet_status.on = magnet_on;
        println!(
            "Set new goal = {:?}, magnet status: {:?}",
            goal, magnet_status.on
        );
        let goal_url = format!(
            "{}/{}/{}/{}",
            env::var("URL").expect("URL not set"),
            goal.x(),
            goal.y(),
            magnet_on
        );
        println!("hier nog ok?");
        reqwest::blocking::get(&goal_url).unwrap();
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
            // player_turn.turn = false;
            let m = current_move.current_move;
            boardstate.chess.perform(m);
            player_turn.turn = !player_turn.turn;
            if player_turn.turn == Player::Computer {
                computer_turn.send(ComputerTurnEvent);
                // return_move(boardstate, player_turn, current_move);
            }
        } else {
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
