use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::structs::{GameName, ItemId, LocationId, SlotId, SlotName, SlotType, TeamId};
use crate::messages::NetworkPlayer;

#[derive(Serialize, Deserialize, Debug)]
pub struct Seed {
    pub slot_data: HashMap<SlotId, HashMap<String, serde_json::Value>>,
    pub slot_info: HashMap<SlotId, (SlotName, GameName, SlotType, Vec<SlotId>)>,
    pub connect_names: HashMap<SlotName, (TeamId, SlotId)>,
    pub locations: HashMap<SlotId, HashMap<LocationId, (ItemId, SlotId, u8 /* item flags */)>>,
    // checks_in_area: serde_json::Value,
    // server_options:  serde_json::Value,
    // er_hint_data:  serde_json::Value,
    precollected_items: HashMap<SlotId, Vec<ItemId>>,
    precollected_hints: HashMap<SlotId, Vec<ItemId>>,
    pub version: (u16, u16, u16),
    // tags: Vec<String>
    minimum_versions: MinimumVersions,
    pub seed_name: String,
    spheres: Vec<HashMap<SlotId, Vec<i64 /*item or location id idk*/>>>,
    pub datapackage: HashMap<GameName, GameDatapackagePart>,
    race_mode: i32,
}

impl Seed {
    pub fn slot_ids(&self) -> Vec<SlotId> {
        self.slot_data.keys().map(|s| s.clone()).collect()
    }
    pub fn games(&self) -> Vec<GameName> {
        self.slot_info.values().map(|(_, game, _, _)| game.clone()).collect()
    }
    pub fn datapackage_checksums(&self) -> HashMap<GameName, String> {
        let mut result = HashMap::new();
        for (game, part) in &self.datapackage {
            result.insert(game.clone(), part.checksum.clone());
        }
        result
    }

    pub fn players(&self) -> Vec<NetworkPlayer> {
        self.slot_info.iter().map(|(slot_id, (slot_name, game_name, slot_type, _))| {
            NetworkPlayer::new(
                TeamId::not_implemented(),
                slot_id.clone(),
                slot_name.clone().to_string(), // TODO implement Aliases
                slot_name.clone(),
            )
        }).collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MinimumVersions {
    server: (u16, u16, u16),
    clients: HashMap<SlotId /*slot id*/, (u16, u16, u16)>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameDatapackagePart {
    item_name_groups: HashMap<String /* group name */, Vec<String /* item name */>>,
    item_name_to_id: HashMap<String, ItemId>,
    location_name_groups: HashMap<String /* group name */, Vec<String /* location name */>>,
    location_name_to_id: HashMap<String, LocationId>,
    checksum: String,
}