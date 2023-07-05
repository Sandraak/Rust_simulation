use crate::{
    chess::{chess::Move, pos::Pos, BoardState},
    pathfinding::astar::Path,
};
use bevy::prelude::*;


pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentPaths>()
        .init_resource::<CurrentLocations>()
        .insert_resource(Destination{goal : Pos { x: 0, y: 0 }})
        .insert_resource(CurrentMove{current_move : Move {from : Pos { x: 0, y: 0 }, to : Pos { x: 0, y: 0 }}})
        .add_system(update_path)
        .add_system(flatten_locations)
        .add_system(update_current_pos)
        .add_system(end_turn);
    }
}

#[derive(Resource, Default)]
pub struct CurrentPaths {
    pub paths: Vec<Path>,
}
#[derive(Resource, Default)]
pub struct CurrentLocations {
    pub locations: Vec<Pos>,
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
    pub first_move: bool,
    pub on: bool,
}

impl Default for MagnetStatus {
    fn default() -> Self {
        Self {
            simulation: false,
            real: false,
            first_move: true,
            on: false,
        }
    }
}
pub struct MoveEvent;
pub struct NewPathEvent;
pub struct PathEvent;
pub struct FirstMoveEvent;
pub struct MagnetEvent;

/// Wanneer de CurrentMove resource verandert, stuurt de chess computer een MoveEvent dat dit is gebeurd.
/// update_path reageert op dit event door een PathEvent te sturen
/// De functie give_path in het Pathfinding component luistert naar dit event en update de CurrentLocations resource.
fn update_path(_new_move: EventReader<MoveEvent>, mut new_path: EventWriter<PathEvent>) {
    new_path.send(PathEvent);
}

fn flatten_locations(
    current_paths: ResMut<CurrentPaths>,
    mut current_locations: ResMut<CurrentLocations>,
    path_update: EventReader<NewPathEvent>,
    mut start_move: EventWriter<FirstMoveEvent>,
) {
    current_locations.locations = current_paths.paths.iter().cloned().flatten().collect();
    start_move.send(FirstMoveEvent);
}

/// wanneer de locatie bereikt is,
/// Update de current locatie naar devolgende in de lijst locaties.
/// Zorg er voor dat de magneten gaan veranderen wanneer deze locatie verandert.
fn update_current_pos(
    first_move: EventReader<FirstMoveEvent>,
    magnet_update: EventReader<MagnetEvent>,
    mut magnet_status: ResMut<MagnetStatus>,
    mut locations_flat: ResMut<CurrentLocations>,
    mut new_pos: ResMut<Destination>,
) {
    if magnet_status.simulation && magnet_status.real {
        *new_pos = Destination {
            goal: *locations_flat.locations.first().unwrap(),
        };
        locations_flat.locations.remove(0);
        magnet_status.simulation = false;
        magnet_status.real = false;
    }
}

/// activate when all the locations have been reached.
/// reset all the resources linked to the current turn.
/// update the boardstate
fn end_turn(
    mut current_locations: ResMut<CurrentPaths>,
    mut status: ResMut<MagnetStatus>,
) {
    *current_locations = CurrentPaths { paths: vec![] };
    status.simulation = false;
    status.real = false;
    status.first_move = true;
    status.on = false;
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
