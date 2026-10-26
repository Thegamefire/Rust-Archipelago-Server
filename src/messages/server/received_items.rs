use crate::messages::NetworkItem;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ReceivedItemsPacket {
    pub index: u64,
    pub items: Vec<NetworkItem>,
}
