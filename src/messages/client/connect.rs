use serde::{Deserialize, Serialize};
use crate::messages::NetworkVersion;
use crate::structs::{GameName, SlotName};

#[derive(Serialize, Deserialize)]
pub struct ConnectPacket {
    pub password: String,
    pub game: GameName,
    pub name: SlotName,
    pub uuid: String,
    pub version: NetworkVersion,
    pub items_handling: u8,
    pub tags: Vec<String>,
    pub slot_data: bool,
}