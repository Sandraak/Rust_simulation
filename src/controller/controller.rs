use crate::{
    chess::{chess::Move, pos::Pos, BoardState},
    pathfinding::astar::Path,
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct CurrentLocations {
    pub path:  Option<Vec<Path>>,
}
#[derive(Resource)]
pub struct Destination {
    pub goal: Pos,
}

#[derive(Resource)]
pub struct CurrentMove {
    pub current_move: Move,
}


pub struct NewMoveEvent;

pub struct NewLocationsEvent;

struct NewPosEvent;

// TODO draaien systems in een aparte thread?
// welke thread is waar bezig?

/// Systeem dat de current move aanpast.
// fn update_move(ev_new_move: EventReader<NewMoveEvent>){

// }

/// 1) Maak een systeem dat een nieuwe pad berekent wanneer de current move verandert.
///        a)Stuur deze Move naar het pathfinding component
///        b) Zorgt er voor dat pathfinding een pad gaat berekenen wanneer hij een nieuwe move krijgt.
fn update_path(mut ev_new_move: EventWriter<NewMoveEvent>) {
    ev_new_move.send(NewMoveEvent);
}

/// Hak de lijst met posities op
fn update_locations(ev_new_locations: EventReader<NewLocationsEvent>) {}
/// wanneer de locatie bereikt is,
/// Update de current locatie naar devolgende in de lijst locaties.
/// Zorg er voor dat de magneten gaan veranderen wanneer deze locatie verandert.
fn update_current_pos(ev_new_pos: EventWriter<NewPosEvent>) {}

/// activate when all the locations have been reached.
/// reset all the resources linked to the current turn.
/// update the boardstate
fn end_turn() {}

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
