use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawSeed {
    slot_data: HashMap<u32, serde_json::Value>,
    slot_info: HashMap<u32, (String, String, u8, Vec<u32>)>,
    connect_names: HashMap<String, (i32, u32 /* slot id */)>,
    locations: HashMap<u32, /*location slot id*/ HashMap<i64 /* Location Id */, (i64 /* item id */, u32 /* item owner */, u8 /* item flags */)>>,
    // checks_in_area: serde_json::Value,
    // server_options:  serde_json::Value,
    // er_hint_data:  serde_json::Value,
    precollected_items: HashMap<u32 /* slot id */, Vec<i64/* item id */>>,
    precollected_hints: HashMap<u32 /* slot id */, Vec<i64/* item id */>>,
    version: (u16, u16, u16),
    // tags: Vec<String>
    minimum_versions: MinimumVersions,
    seed_name: String,
    spheres: Vec<HashMap<u32, Vec<i64 /*item or location id idk*/>>>,
    datapackage: HashMap<String, GameDatapackagePart>,
    race_mode: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct MinimumVersions {
    server: (u16, u16, u16),
    clients: HashMap<u32/*slot id*/, (u16, u16, u16)>
}

#[derive(Serialize, Deserialize, Debug)]
struct GameDatapackagePart {
    item_name_groups: HashMap<String /* group name */, Vec<String /* item name */>>,
    item_name_to_id: HashMap<String, i64>,
    location_name_groups: HashMap<String /* group name */, Vec<String /* location name */>>,
    location_name_to_id: HashMap<String, i64>,
    checksum: String,
}