use crate::messages::{NetworkVersion, Permission};
use crate::seed::Seed;
use crate::state::MultiworldState;
use crate::structs::GameName;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct RoomInfoPacket {
    version: NetworkVersion,
    generator_version: NetworkVersion,
    tags: Vec<String>,
    password: bool,
    permissions: HashMap<String, Permission>,
    hint_cost: u64,
    location_check_points: u64,
    games: Vec<GameName>,
    datapackage_checksums: HashMap<GameName, String>,
    seed_name: String,
    time: f64,
}
impl RoomInfoPacket {
    pub fn new(seed: &Seed, _multiworld_state: &MultiworldState) -> RoomInfoPacket {
        RoomInfoPacket {
            version: NetworkVersion::new(0, 0, 1),
            generator_version: NetworkVersion::new(seed.version.0, seed.version.1, seed.version.2),
            tags: vec!["ARS".into()],
            password: false,
            permissions: Default::default(),
            hint_cost: 0,
            location_check_points: 0,
            games: seed.games(),
            datapackage_checksums: seed.datapackage_checksums(),
            seed_name: seed.seed_name.clone(),
            time: 0.0,
        }
    }
}
