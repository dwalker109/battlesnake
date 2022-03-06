use crate::snake::Dir;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub game: Game,
    turn: u32,
    board: Board,
    pub you: Battlesnake,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponse {
    pub apiversion: &'static str,
    pub author: &'static str,
    pub color: &'static str,
    pub head: &'static str,
    pub tail: &'static str,
    pub version: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveResponse {
    r#move: Dir,
}

impl From<Dir> for MoveResponse {
    fn from(dir: Dir) -> Self {
        Self { r#move: dir }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: String,
    ruleset: Ruleset,
    timeout: u32,
    source: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Ruleset {
    name: String,
    version: String,
    settings: RulesetSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RulesetSettings {
    food_spawn_chance: u32,
    minimum_food: u32,
    hazard_damage_per_turn: u32,
    hazard_map: String,
    hazard_map_author: String,
    royale: RulesetRoyaleSettings,
    squad: RulesetSquadSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RulesetRoyaleSettings {
    shrink_every_n_turns: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RulesetSquadSettings {
    allow_body_collisions: bool,
    shared_elimination: bool,
    shared_health: bool,
    shared_length: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battlesnake {
    pub id: String,
    name: String,
    health: u32,
    body: Vec<Coords>,
    latency: String,
    head: Coords,
    length: u32,
    shout: String,
    squad: String,
    customizations: BattlesnakeCustomizations,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BattlesnakeCustomizations {
    color: String,
    head: String,
    tail: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    height: u32,
    width: u32,
    food: Vec<Coords>,
    hazards: Vec<Coords>,
    snakes: Vec<Battlesnake>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Coords {
    x: u32,
    y: u32,
}
