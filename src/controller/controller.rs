use bevy::prelude::*;
use crate::{pathfinding::astar::Path, chess::{pos::Pos, chess::Move}};


/// TODO:
/// 1) Maak een systeem dat een nieuwe pad berekent wanneer de current move verandert.
/// 2) Stuur deze Move naar het pathfinding component
///     a) Zorgt er voor dat pathfinding een pad gaat berekenen wanneer hij een nieuwe move krijgt.
/// 3) Hak dit pad op in locaties
///     a) stuur deze locaties 1 voor 1 naar zowel hardware als simulatie
///     b) wanneer de laatste locatie bereikt is, reset alles.
/// 
/// Maak een systeem dat de volgende positie naar de simulatie en hardware stuurt.
/// Hij moet dit doen zodra de magneet op beide plekken op de juiste positie is.
/// Meegeven of de magneet uit of aan is.
/// 
/// In magnet.rs zitten 2 (3?) systemen die hier interactie mee hebben.
/// 1) Systeem dat reageert wanneer het een nieuwe positie krijgt en deze update naar de ontvangen waarde.
/// 2) Systeem dat wanneer de magneet niet op deze positie is, naar deze positie beweegt.
/// 3) Systeem dat een bericht naar de controller stuurt wanneer de positie bereikt is.
/// 
/// De hardware bevat dezelfde systemen.


#[derive(Resource)]
pub struct Locations{
    path: Vec<Path>,
}

#[derive(Resource)]
pub struct CurrentGoal{
    path: Pos,
}

#[derive(Resource)]
pub struct CurrentMove{
    mov : Move,
}

