use serde::{Deserialize, Serialize};
use crate::messages::NetworkItem;

#[derive(Deserialize, Serialize)]
pub struct ReceivedItemsPacket {
    pub index: u64,
    pub items: Vec<NetworkItem>
}