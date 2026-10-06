use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::structs::{ItemId, LocationId, SlotId};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawSeed {
    slot_data: HashMap<SlotId, serde_json::Value>,
    slot_info: HashMap<SlotId, (String, String, u8, Vec<SlotId>)>,
    connect_names: HashMap<String, (i32, SlotId /* slot id */)>,
    locations: HashMap<SlotId, /*location slot id*/ HashMap<LocationId /* Location Id */, (ItemId /* item id */, u32 /* item owner */, u8 /* item flags */)>>,
    // checks_in_area: serde_json::Value,
    // server_options:  serde_json::Value,
    // er_hint_data:  serde_json::Value,
    precollected_items: HashMap<SlotId /* slot id */, Vec<ItemId/* item id */>>,
    precollected_hints: HashMap<SlotId /* slot id */, Vec<ItemId/* item id */>>,
    version: (u16, u16, u16),
    // tags: Vec<String>
    minimum_versions: MinimumVersions,
    seed_name: String,
    spheres: Vec<HashMap<SlotId, Vec<i64 /*item or location id idk*/>>>,
    datapackage: HashMap<String, GameDatapackagePart>,
    race_mode: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct MinimumVersions {
    server: (u16, u16, u16),
    clients: HashMap<SlotId /*slot id*/, (u16, u16, u16)>
}

#[derive(Serialize, Deserialize, Debug)]
struct GameDatapackagePart {
    item_name_groups: HashMap<String /* group name */, Vec<String /* item name */>>,
    item_name_to_id: HashMap<String, ItemId>,
    location_name_groups: HashMap<String /* group name */, Vec<String /* location name */>>,
    location_name_to_id: HashMap<String, LocationId>,
    checksum: String,
}