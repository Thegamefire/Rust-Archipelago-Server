use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::messages::NetworkPlayer;
use crate::structs::{GameName, LocationId, SlotId, SlotName, SlotType, TeamId};


#[derive(Serialize, Deserialize)]
pub struct ConnectedPacket {
    pub team: TeamId,
    pub slot: SlotId,
    pub players: Vec<NetworkPlayer>,
    pub missing_locations: Vec<LocationId>,
    pub checked_locations: Vec<LocationId>,
    pub slot_data: HashMap<String, serde_json::Value>,
    pub slot_info: (SlotName, GameName, SlotType, Vec<SlotId>),
    pub hint_points: u64,
}