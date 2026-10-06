use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::messages::{NetworkVersion, Permission};

#[derive(Deserialize, Serialize)]
struct RoomInfoPacket {
    version: NetworkVersion,
    generator_version: NetworkVersion,
    tags: Vec<String>,
    password: bool,
    permissions: HashMap<String, Permission>,
    hint_cost: u64,
    location_check_points: u64,
    games: Vec<String>,
    datapackage_checksums: HashMap<String, String>,
    seed_name: String,
    time: f64,
}


