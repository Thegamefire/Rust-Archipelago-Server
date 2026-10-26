use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ConnectUpdatePacket {
    pub items_handling: u8,
    pub tags: Vec<String>,
}
