struct Game {
    id: String,
    ruleset: Ruleset,
    timeout: u32,
    source: String,
}

struct Ruleset {
    name: String,
    version: String,
    settings: RulesetSettings,
}

struct RulesetSettings {
    food_spawn_chance: u32,
    minimum_food: u32,
    hazard_damage_per_turn: u32,
    map: String,
    royale: RulesetRoyaleSettings,
    squas: RulesetSquadSettings,
}

struct RulesetRoyaleSettings {
    shrink_every_n_turns: u32,
}

struct RulesetSquadSettings {
    allow_body_collisions: bool,
    shared_elimination: bool,
    shared_health: bool,
    shared_length: bool,
}

struct Battlesnake {
    id: String,
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

struct BattlesnakeCustomizations {
    color: String,
    head: String,
    tail: String,
}

struct Board {
    height: u32,
    width: u32,
    food: Vec<Coords>,
    hazards: Vec<Coords>,
    snakes: Vec<Battlesnake>,
}

struct Coords {
    x: u32,
    y: u32,
}

