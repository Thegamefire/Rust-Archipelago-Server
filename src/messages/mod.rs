use serde::{Deserialize, Serialize, Serializer};
use crate::structs::{SlotId, SlotName, TeamId};

pub mod client;
pub mod server;

#[derive(Serialize, Deserialize)]
pub struct NetworkVersion {
    class: String,
    major: u16,
    minor: u16,
    build: u16,
}
impl NetworkVersion {
    fn new(major: u16, minor: u16, build: u16) -> Self {
        NetworkVersion {
            class: String::from("Version"),
            major,
            minor,
            build,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct NetworkPlayer {
    class: String,
    pub team: TeamId,
    pub slot: SlotId,
    pub alias: String,
    pub name: SlotName,
}
impl NetworkPlayer {
    pub fn new(team: TeamId, slot: SlotId, alias: String, name: SlotName) -> Self {
        NetworkPlayer {
            class: String::from("NetworkPlayer"),
            team,
            slot,
            alias,
            name,
        }
    }
}

#[derive(Deserialize, Serialize)]
enum Permission {
    Disabled, Enabled, Goal, Auto, AutoEnabled
}